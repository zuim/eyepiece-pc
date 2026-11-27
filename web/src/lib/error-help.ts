// What to do about an error the engine reported, where the message alone does not say it:
// a sentence and a list. null when there is nothing to add.

export interface ErrorHelp {
  lead: string;
  steps: string[];
}

const IN_USE = /goggles' (video|control) interface is in use by another program/;

export function errorHelp(message: string): ErrorHelp | null {
  if (IN_USE.test(message))
    return {
      lead: "Close it, then start again:",
      steps: [
        "a second eyepiece-pc",
        "a browser tab streaming from tinkerglass' PC Display panel",
        "a libusb tool such as tg connect",
        "none of these: replug the goggles",
      ],
    };
  return null;
}
