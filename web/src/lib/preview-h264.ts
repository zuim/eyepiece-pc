export function nalUnits(data: Uint8Array): Uint8Array[] {
  const starts: Array<{ index: number; size: number }> = [];
  for (let i = 0; i + 3 <= data.length;) {
    if (data[i] === 0 && data[i + 1] === 0 && data[i + 2] === 1) {
      starts.push({ index: i, size: 3 });
      i += 3;
    } else if (i + 4 <= data.length && data[i] === 0 && data[i + 1] === 0 && data[i + 2] === 0 && data[i + 3] === 1) {
      starts.push({ index: i, size: 4 });
      i += 4;
    } else i++;
  }
  return starts
    .map((start, index) => {
      const end = starts[index + 1]?.index ?? data.length;
      return data.subarray(start.index + start.size, end);
    })
    .filter((unit) => unit.length > 0);
}

export function avcDescription(units: Uint8Array[]): { codec: string; description: Uint8Array } | undefined {
  const sps = units.find((unit) => (unit[0] & 0x1f) === 7);
  const pps = units.find((unit) => (unit[0] & 0x1f) === 8);
  if (!sps || !pps || sps.length < 4) return undefined;

  const description = new Uint8Array(11 + sps.length + pps.length);
  let offset = 0;
  description[offset++] = 1;
  description[offset++] = sps[1];
  description[offset++] = sps[2];
  description[offset++] = sps[3];
  description[offset++] = 0xff;
  description[offset++] = 0xe1;
  description[offset++] = (sps.length >> 8) & 0xff;
  description[offset++] = sps.length & 0xff;
  description.set(sps, offset);
  offset += sps.length;
  description[offset++] = 1;
  description[offset++] = (pps.length >> 8) & 0xff;
  description[offset++] = pps.length & 0xff;
  description.set(pps, offset);

  return {
    codec: `avc1.${[sps[1], sps[2], sps[3]].map((byte) => byte.toString(16).padStart(2, "0")).join("")}`,
    description,
  };
}

export function lengthPrefixed(units: Uint8Array[]): Uint8Array {
  const size = units.reduce((total, unit) => total + 4 + unit.length, 0);
  const output = new Uint8Array(size);
  const view = new DataView(output.buffer);
  let offset = 0;
  for (const unit of units) {
    view.setUint32(offset, unit.length, false);
    output.set(unit, offset + 4);
    offset += 4 + unit.length;
  }
  return output;
}
