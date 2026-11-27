import * as $protobuf from "protobufjs";
import Long = require("long");
/** Namespace eyepiece. */
export namespace eyepiece {

    /** Namespace rpc. */
    namespace rpc {

        /** Namespace v1. */
        namespace v1 {

            /** Codec enum. */
            enum Codec {
                CODEC_H264 = 0,
                CODEC_HEVC = 1
            }

            /** ControlOperation enum. */
            enum ControlOperation {
                CONTROL_UNSPECIFIED = 0,
                CONTROL_RECENTER = 1,
                CONTROL_QUIT = 2,
                CONTROL_HEADTRACK = 3
            }

            /** StreamLayout enum. */
            enum StreamLayout {
                LAYOUT_MONO = 0,
                LAYOUT_SBS = 1
            }

            /** PaneFormat enum. */
            enum PaneFormat {
                PANE_FORMAT_UNSPECIFIED = 0,
                PANE_BGRA = 1,
                PANE_OPAQUE_BGRA = 2
            }

            /** PoseSource enum. */
            enum PoseSource {
                POSE_SOURCE_UNSPECIFIED = 0,
                POSE_SOURCE_MCU = 3
            }

            /** BridgeCapability enum. */
            enum BridgeCapability {
                CAPABILITY_NONE = 0,
                CAPABILITY_STEREO = 1,
                CAPABILITY_PANE_BGRA = 4,
                CAPABILITY_PANE_OPAQUE_BGRA = 8,
                CAPABILITY_RECONFIG = 32,
                CAPABILITY_GUI_ALPHA = 64,
                CAPABILITY_PANE_ALPHA = 256
            }

            /** Properties of an Envelope. */
            interface IEnvelope {

                /** Envelope id */
                id?: (string|null);

                /** Envelope sender */
                sender?: (string|null);

                /** Envelope recipient */
                recipient?: (string|null);

                /** Envelope getState */
                getState?: (eyepiece.rpc.v1.IGetState|null);

                /** Envelope setProperties */
                setProperties?: (eyepiece.rpc.v1.ISetProperties|null);

                /** Envelope start */
                start?: (eyepiece.rpc.v1.IStart|null);

                /** Envelope stop */
                stop?: (eyepiece.rpc.v1.IStop|null);

                /** Envelope restart */
                restart?: (eyepiece.rpc.v1.IRestart|null);

                /** Envelope recenter */
                recenter?: (eyepiece.rpc.v1.IRecenter|null);

                /** Envelope forceIdr */
                forceIdr?: (eyepiece.rpc.v1.IForceIdr|null);

                /** Envelope setPreview */
                setPreview?: (eyepiece.rpc.v1.ISetPreview|null);

                /** Envelope setCommandLine */
                setCommandLine?: (eyepiece.rpc.v1.ISetCommandLine|null);

                /** Envelope listKmsOutputs */
                listKmsOutputs?: (eyepiece.rpc.v1.IListKmsOutputs|null);

                /** Envelope state */
                state?: (eyepiece.rpc.v1.IState|null);

                /** Envelope error */
                error?: (eyepiece.rpc.v1.IError|null);

                /** Envelope log */
                log?: (eyepiece.rpc.v1.ILog|null);

                /** Envelope pose */
                pose?: (eyepiece.rpc.v1.IPose|null);

                /** Envelope metrics */
                metrics?: (eyepiece.rpc.v1.IMetrics|null);

                /** Envelope preview */
                preview?: (eyepiece.rpc.v1.IPreview|null);

                /** Envelope acknowledged */
                acknowledged?: (boolean|null);

                /** Envelope kmsOutputs */
                kmsOutputs?: (eyepiece.rpc.v1.IKmsOutputs|null);

                /** Envelope video */
                video?: (eyepiece.rpc.v1.IVideo|null);

                /** Envelope ping */
                ping?: (eyepiece.rpc.v1.IPing|null);

                /** Envelope control */
                control?: (eyepiece.rpc.v1.IControl|null);

                /** Envelope guiAlpha */
                guiAlpha?: (eyepiece.rpc.v1.IGuiAlpha|null);

                /** Envelope pong */
                pong?: (eyepiece.rpc.v1.IPong|null);

                /** Envelope frame */
                frame?: (eyepiece.rpc.v1.IFrame|null);

                /** Envelope info */
                info?: (eyepiece.rpc.v1.IInfo|null);

                /** Envelope keyRequest */
                keyRequest?: (eyepiece.rpc.v1.IKeyRequest|null);

                /** Envelope guiAlphaAck */
                guiAlphaAck?: (eyepiece.rpc.v1.IAcknowledgement|null);

                /** Envelope paneAlpha */
                paneAlpha?: (eyepiece.rpc.v1.IPaneAlpha|null);

                /** Envelope paneAlphaAck */
                paneAlphaAck?: (eyepiece.rpc.v1.IAcknowledgement|null);

                /** Envelope configure */
                configure?: (eyepiece.rpc.v1.IConfigure|null);

                /** Envelope configureAck */
                configureAck?: (eyepiece.rpc.v1.IAcknowledgement|null);
            }

            /** Represents an Envelope. */
            class Envelope implements IEnvelope {

                /**
                 * Constructs a new Envelope.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IEnvelope);

                /** Envelope id. */
                public id?: (string|null);

                /** Envelope sender. */
                public sender?: (string|null);

                /** Envelope recipient. */
                public recipient?: (string|null);

                /** Envelope getState. */
                public getState?: (eyepiece.rpc.v1.IGetState|null);

                /** Envelope setProperties. */
                public setProperties?: (eyepiece.rpc.v1.ISetProperties|null);

                /** Envelope start. */
                public start?: (eyepiece.rpc.v1.IStart|null);

                /** Envelope stop. */
                public stop?: (eyepiece.rpc.v1.IStop|null);

                /** Envelope restart. */
                public restart?: (eyepiece.rpc.v1.IRestart|null);

                /** Envelope recenter. */
                public recenter?: (eyepiece.rpc.v1.IRecenter|null);

                /** Envelope forceIdr. */
                public forceIdr?: (eyepiece.rpc.v1.IForceIdr|null);

                /** Envelope setPreview. */
                public setPreview?: (eyepiece.rpc.v1.ISetPreview|null);

                /** Envelope setCommandLine. */
                public setCommandLine?: (eyepiece.rpc.v1.ISetCommandLine|null);

                /** Envelope listKmsOutputs. */
                public listKmsOutputs?: (eyepiece.rpc.v1.IListKmsOutputs|null);

                /** Envelope state. */
                public state?: (eyepiece.rpc.v1.IState|null);

                /** Envelope error. */
                public error?: (eyepiece.rpc.v1.IError|null);

                /** Envelope log. */
                public log?: (eyepiece.rpc.v1.ILog|null);

                /** Envelope pose. */
                public pose?: (eyepiece.rpc.v1.IPose|null);

                /** Envelope metrics. */
                public metrics?: (eyepiece.rpc.v1.IMetrics|null);

                /** Envelope preview. */
                public preview?: (eyepiece.rpc.v1.IPreview|null);

                /** Envelope acknowledged. */
                public acknowledged?: (boolean|null);

                /** Envelope kmsOutputs. */
                public kmsOutputs?: (eyepiece.rpc.v1.IKmsOutputs|null);

                /** Envelope video. */
                public video?: (eyepiece.rpc.v1.IVideo|null);

                /** Envelope ping. */
                public ping?: (eyepiece.rpc.v1.IPing|null);

                /** Envelope control. */
                public control?: (eyepiece.rpc.v1.IControl|null);

                /** Envelope guiAlpha. */
                public guiAlpha?: (eyepiece.rpc.v1.IGuiAlpha|null);

                /** Envelope pong. */
                public pong?: (eyepiece.rpc.v1.IPong|null);

                /** Envelope frame. */
                public frame?: (eyepiece.rpc.v1.IFrame|null);

                /** Envelope info. */
                public info?: (eyepiece.rpc.v1.IInfo|null);

                /** Envelope keyRequest. */
                public keyRequest?: (eyepiece.rpc.v1.IKeyRequest|null);

                /** Envelope guiAlphaAck. */
                public guiAlphaAck?: (eyepiece.rpc.v1.IAcknowledgement|null);

                /** Envelope paneAlpha. */
                public paneAlpha?: (eyepiece.rpc.v1.IPaneAlpha|null);

                /** Envelope paneAlphaAck. */
                public paneAlphaAck?: (eyepiece.rpc.v1.IAcknowledgement|null);

                /** Envelope configure. */
                public configure?: (eyepiece.rpc.v1.IConfigure|null);

                /** Envelope configureAck. */
                public configureAck?: (eyepiece.rpc.v1.IAcknowledgement|null);

                /** Envelope message. */
                public message?: ("getState"|"setProperties"|"start"|"stop"|"restart"|"recenter"|"forceIdr"|"setPreview"|"setCommandLine"|"listKmsOutputs"|"state"|"error"|"log"|"pose"|"metrics"|"preview"|"acknowledged"|"kmsOutputs"|"video"|"ping"|"control"|"guiAlpha"|"pong"|"frame"|"info"|"keyRequest"|"guiAlphaAck"|"paneAlpha"|"paneAlphaAck"|"configure"|"configureAck");

                /**
                 * Creates a new Envelope instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Envelope instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IEnvelope): eyepiece.rpc.v1.Envelope;

                /**
                 * Encodes the specified Envelope message. Does not implicitly {@link eyepiece.rpc.v1.Envelope.verify|verify} messages.
                 * @param message Envelope message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IEnvelope, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Envelope message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Envelope.verify|verify} messages.
                 * @param message Envelope message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IEnvelope, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes an Envelope message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Envelope
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.Envelope;

                /**
                 * Decodes an Envelope message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Envelope
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.Envelope;

                /**
                 * Verifies an Envelope message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates an Envelope message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Envelope
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.Envelope;

                /**
                 * Creates a plain object from an Envelope message. Also converts values to other types if specified.
                 * @param message Envelope
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.Envelope, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Envelope to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for Envelope
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a StringList. */
            interface IStringList {

                /** StringList values */
                values?: (string[]|null);
            }

            /** Represents a StringList. */
            class StringList implements IStringList {

                /**
                 * Constructs a new StringList.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IStringList);

                /** StringList values. */
                public values: string[];

                /**
                 * Creates a new StringList instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns StringList instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IStringList): eyepiece.rpc.v1.StringList;

                /**
                 * Encodes the specified StringList message. Does not implicitly {@link eyepiece.rpc.v1.StringList.verify|verify} messages.
                 * @param message StringList message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IStringList, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified StringList message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.StringList.verify|verify} messages.
                 * @param message StringList message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IStringList, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a StringList message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns StringList
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.StringList;

                /**
                 * Decodes a StringList message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns StringList
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.StringList;

                /**
                 * Verifies a StringList message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a StringList message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns StringList
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.StringList;

                /**
                 * Creates a plain object from a StringList message. Also converts values to other types if specified.
                 * @param message StringList
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.StringList, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this StringList to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for StringList
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a Value. */
            interface IValue {

                /** Value boolean */
                boolean?: (boolean|null);

                /** Value integer */
                integer?: (number|Long|null);

                /** Value number */
                number?: (number|null);

                /** Value text */
                text?: (string|null);

                /** Value strings */
                strings?: (eyepiece.rpc.v1.IStringList|null);

                /** Value unset */
                unset?: (boolean|null);
            }

            /** Represents a Value. */
            class Value implements IValue {

                /**
                 * Constructs a new Value.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IValue);

                /** Value boolean. */
                public boolean?: (boolean|null);

                /** Value integer. */
                public integer?: (number|Long|null);

                /** Value number. */
                public number?: (number|null);

                /** Value text. */
                public text?: (string|null);

                /** Value strings. */
                public strings?: (eyepiece.rpc.v1.IStringList|null);

                /** Value unset. */
                public unset?: (boolean|null);

                /** Value kind. */
                public kind?: ("boolean"|"integer"|"number"|"text"|"strings"|"unset");

                /**
                 * Creates a new Value instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Value instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IValue): eyepiece.rpc.v1.Value;

                /**
                 * Encodes the specified Value message. Does not implicitly {@link eyepiece.rpc.v1.Value.verify|verify} messages.
                 * @param message Value message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IValue, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Value message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Value.verify|verify} messages.
                 * @param message Value message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IValue, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Value message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Value
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.Value;

                /**
                 * Decodes a Value message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Value
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.Value;

                /**
                 * Verifies a Value message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Value message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Value
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.Value;

                /**
                 * Creates a plain object from a Value message. Also converts values to other types if specified.
                 * @param message Value
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.Value, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Value to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for Value
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a Property. */
            interface IProperty {

                /** Property name */
                name?: (string|null);

                /** Property value */
                value?: (eyepiece.rpc.v1.IValue|null);
            }

            /** Represents a Property. */
            class Property implements IProperty {

                /**
                 * Constructs a new Property.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IProperty);

                /** Property name. */
                public name: string;

                /** Property value. */
                public value?: (eyepiece.rpc.v1.IValue|null);

                /**
                 * Creates a new Property instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Property instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IProperty): eyepiece.rpc.v1.Property;

                /**
                 * Encodes the specified Property message. Does not implicitly {@link eyepiece.rpc.v1.Property.verify|verify} messages.
                 * @param message Property message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IProperty, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Property message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Property.verify|verify} messages.
                 * @param message Property message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IProperty, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Property message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Property
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.Property;

                /**
                 * Decodes a Property message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Property
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.Property;

                /**
                 * Verifies a Property message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Property message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Property
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.Property;

                /**
                 * Creates a plain object from a Property message. Also converts values to other types if specified.
                 * @param message Property
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.Property, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Property to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for Property
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a GetState. */
            interface IGetState {
            }

            /** Represents a GetState. */
            class GetState implements IGetState {

                /**
                 * Constructs a new GetState.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IGetState);

                /**
                 * Creates a new GetState instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns GetState instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IGetState): eyepiece.rpc.v1.GetState;

                /**
                 * Encodes the specified GetState message. Does not implicitly {@link eyepiece.rpc.v1.GetState.verify|verify} messages.
                 * @param message GetState message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IGetState, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified GetState message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.GetState.verify|verify} messages.
                 * @param message GetState message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IGetState, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a GetState message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns GetState
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.GetState;

                /**
                 * Decodes a GetState message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns GetState
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.GetState;

                /**
                 * Verifies a GetState message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a GetState message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns GetState
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.GetState;

                /**
                 * Creates a plain object from a GetState message. Also converts values to other types if specified.
                 * @param message GetState
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.GetState, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this GetState to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for GetState
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a SetProperties. */
            interface ISetProperties {

                /** SetProperties properties */
                properties?: (eyepiece.rpc.v1.IProperty[]|null);
            }

            /** Represents a SetProperties. */
            class SetProperties implements ISetProperties {

                /**
                 * Constructs a new SetProperties.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.ISetProperties);

                /** SetProperties properties. */
                public properties: eyepiece.rpc.v1.IProperty[];

                /**
                 * Creates a new SetProperties instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns SetProperties instance
                 */
                public static create(properties?: eyepiece.rpc.v1.ISetProperties): eyepiece.rpc.v1.SetProperties;

                /**
                 * Encodes the specified SetProperties message. Does not implicitly {@link eyepiece.rpc.v1.SetProperties.verify|verify} messages.
                 * @param message SetProperties message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.ISetProperties, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified SetProperties message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.SetProperties.verify|verify} messages.
                 * @param message SetProperties message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.ISetProperties, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a SetProperties message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns SetProperties
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.SetProperties;

                /**
                 * Decodes a SetProperties message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns SetProperties
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.SetProperties;

                /**
                 * Verifies a SetProperties message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a SetProperties message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns SetProperties
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.SetProperties;

                /**
                 * Creates a plain object from a SetProperties message. Also converts values to other types if specified.
                 * @param message SetProperties
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.SetProperties, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this SetProperties to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for SetProperties
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a Start. */
            interface IStart {
            }

            /** Represents a Start. */
            class Start implements IStart {

                /**
                 * Constructs a new Start.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IStart);

                /**
                 * Creates a new Start instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Start instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IStart): eyepiece.rpc.v1.Start;

                /**
                 * Encodes the specified Start message. Does not implicitly {@link eyepiece.rpc.v1.Start.verify|verify} messages.
                 * @param message Start message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IStart, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Start message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Start.verify|verify} messages.
                 * @param message Start message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IStart, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Start message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Start
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.Start;

                /**
                 * Decodes a Start message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Start
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.Start;

                /**
                 * Verifies a Start message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Start message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Start
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.Start;

                /**
                 * Creates a plain object from a Start message. Also converts values to other types if specified.
                 * @param message Start
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.Start, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Start to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for Start
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a Stop. */
            interface IStop {
            }

            /** Represents a Stop. */
            class Stop implements IStop {

                /**
                 * Constructs a new Stop.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IStop);

                /**
                 * Creates a new Stop instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Stop instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IStop): eyepiece.rpc.v1.Stop;

                /**
                 * Encodes the specified Stop message. Does not implicitly {@link eyepiece.rpc.v1.Stop.verify|verify} messages.
                 * @param message Stop message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IStop, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Stop message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Stop.verify|verify} messages.
                 * @param message Stop message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IStop, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Stop message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Stop
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.Stop;

                /**
                 * Decodes a Stop message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Stop
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.Stop;

                /**
                 * Verifies a Stop message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Stop message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Stop
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.Stop;

                /**
                 * Creates a plain object from a Stop message. Also converts values to other types if specified.
                 * @param message Stop
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.Stop, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Stop to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for Stop
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a Restart. */
            interface IRestart {
            }

            /** Represents a Restart. */
            class Restart implements IRestart {

                /**
                 * Constructs a new Restart.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IRestart);

                /**
                 * Creates a new Restart instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Restart instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IRestart): eyepiece.rpc.v1.Restart;

                /**
                 * Encodes the specified Restart message. Does not implicitly {@link eyepiece.rpc.v1.Restart.verify|verify} messages.
                 * @param message Restart message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IRestart, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Restart message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Restart.verify|verify} messages.
                 * @param message Restart message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IRestart, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Restart message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Restart
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.Restart;

                /**
                 * Decodes a Restart message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Restart
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.Restart;

                /**
                 * Verifies a Restart message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Restart message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Restart
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.Restart;

                /**
                 * Creates a plain object from a Restart message. Also converts values to other types if specified.
                 * @param message Restart
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.Restart, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Restart to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for Restart
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a Recenter. */
            interface IRecenter {
            }

            /** Represents a Recenter. */
            class Recenter implements IRecenter {

                /**
                 * Constructs a new Recenter.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IRecenter);

                /**
                 * Creates a new Recenter instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Recenter instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IRecenter): eyepiece.rpc.v1.Recenter;

                /**
                 * Encodes the specified Recenter message. Does not implicitly {@link eyepiece.rpc.v1.Recenter.verify|verify} messages.
                 * @param message Recenter message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IRecenter, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Recenter message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Recenter.verify|verify} messages.
                 * @param message Recenter message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IRecenter, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Recenter message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Recenter
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.Recenter;

                /**
                 * Decodes a Recenter message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Recenter
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.Recenter;

                /**
                 * Verifies a Recenter message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Recenter message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Recenter
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.Recenter;

                /**
                 * Creates a plain object from a Recenter message. Also converts values to other types if specified.
                 * @param message Recenter
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.Recenter, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Recenter to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for Recenter
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a ForceIdr. */
            interface IForceIdr {
            }

            /** Represents a ForceIdr. */
            class ForceIdr implements IForceIdr {

                /**
                 * Constructs a new ForceIdr.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IForceIdr);

                /**
                 * Creates a new ForceIdr instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns ForceIdr instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IForceIdr): eyepiece.rpc.v1.ForceIdr;

                /**
                 * Encodes the specified ForceIdr message. Does not implicitly {@link eyepiece.rpc.v1.ForceIdr.verify|verify} messages.
                 * @param message ForceIdr message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IForceIdr, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified ForceIdr message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.ForceIdr.verify|verify} messages.
                 * @param message ForceIdr message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IForceIdr, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a ForceIdr message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns ForceIdr
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.ForceIdr;

                /**
                 * Decodes a ForceIdr message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns ForceIdr
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.ForceIdr;

                /**
                 * Verifies a ForceIdr message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a ForceIdr message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns ForceIdr
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.ForceIdr;

                /**
                 * Creates a plain object from a ForceIdr message. Also converts values to other types if specified.
                 * @param message ForceIdr
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.ForceIdr, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this ForceIdr to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for ForceIdr
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a SetPreview. */
            interface ISetPreview {

                /** SetPreview enabled */
                enabled?: (boolean|null);

                /** SetPreview supportedCodecs */
                supportedCodecs?: (string[]|null);

                /** SetPreview keyframesOnly */
                keyframesOnly?: (boolean|null);
            }

            /** Represents a SetPreview. */
            class SetPreview implements ISetPreview {

                /**
                 * Constructs a new SetPreview.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.ISetPreview);

                /** SetPreview enabled. */
                public enabled: boolean;

                /** SetPreview supportedCodecs. */
                public supportedCodecs: string[];

                /** SetPreview keyframesOnly. */
                public keyframesOnly: boolean;

                /**
                 * Creates a new SetPreview instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns SetPreview instance
                 */
                public static create(properties?: eyepiece.rpc.v1.ISetPreview): eyepiece.rpc.v1.SetPreview;

                /**
                 * Encodes the specified SetPreview message. Does not implicitly {@link eyepiece.rpc.v1.SetPreview.verify|verify} messages.
                 * @param message SetPreview message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.ISetPreview, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified SetPreview message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.SetPreview.verify|verify} messages.
                 * @param message SetPreview message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.ISetPreview, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a SetPreview message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns SetPreview
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.SetPreview;

                /**
                 * Decodes a SetPreview message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns SetPreview
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.SetPreview;

                /**
                 * Verifies a SetPreview message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a SetPreview message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns SetPreview
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.SetPreview;

                /**
                 * Creates a plain object from a SetPreview message. Also converts values to other types if specified.
                 * @param message SetPreview
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.SetPreview, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this SetPreview to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for SetPreview
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a SetCommandLine. */
            interface ISetCommandLine {

                /** SetCommandLine commandLine */
                commandLine?: (string|null);
            }

            /** Represents a SetCommandLine. */
            class SetCommandLine implements ISetCommandLine {

                /**
                 * Constructs a new SetCommandLine.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.ISetCommandLine);

                /** SetCommandLine commandLine. */
                public commandLine: string;

                /**
                 * Creates a new SetCommandLine instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns SetCommandLine instance
                 */
                public static create(properties?: eyepiece.rpc.v1.ISetCommandLine): eyepiece.rpc.v1.SetCommandLine;

                /**
                 * Encodes the specified SetCommandLine message. Does not implicitly {@link eyepiece.rpc.v1.SetCommandLine.verify|verify} messages.
                 * @param message SetCommandLine message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.ISetCommandLine, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified SetCommandLine message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.SetCommandLine.verify|verify} messages.
                 * @param message SetCommandLine message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.ISetCommandLine, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a SetCommandLine message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns SetCommandLine
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.SetCommandLine;

                /**
                 * Decodes a SetCommandLine message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns SetCommandLine
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.SetCommandLine;

                /**
                 * Verifies a SetCommandLine message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a SetCommandLine message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns SetCommandLine
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.SetCommandLine;

                /**
                 * Creates a plain object from a SetCommandLine message. Also converts values to other types if specified.
                 * @param message SetCommandLine
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.SetCommandLine, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this SetCommandLine to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for SetCommandLine
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a ListKmsOutputs. */
            interface IListKmsOutputs {
            }

            /** Represents a ListKmsOutputs. */
            class ListKmsOutputs implements IListKmsOutputs {

                /**
                 * Constructs a new ListKmsOutputs.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IListKmsOutputs);

                /**
                 * Creates a new ListKmsOutputs instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns ListKmsOutputs instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IListKmsOutputs): eyepiece.rpc.v1.ListKmsOutputs;

                /**
                 * Encodes the specified ListKmsOutputs message. Does not implicitly {@link eyepiece.rpc.v1.ListKmsOutputs.verify|verify} messages.
                 * @param message ListKmsOutputs message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IListKmsOutputs, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified ListKmsOutputs message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.ListKmsOutputs.verify|verify} messages.
                 * @param message ListKmsOutputs message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IListKmsOutputs, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a ListKmsOutputs message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns ListKmsOutputs
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.ListKmsOutputs;

                /**
                 * Decodes a ListKmsOutputs message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns ListKmsOutputs
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.ListKmsOutputs;

                /**
                 * Verifies a ListKmsOutputs message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a ListKmsOutputs message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns ListKmsOutputs
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.ListKmsOutputs;

                /**
                 * Creates a plain object from a ListKmsOutputs message. Also converts values to other types if specified.
                 * @param message ListKmsOutputs
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.ListKmsOutputs, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this ListKmsOutputs to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for ListKmsOutputs
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a State. */
            interface IState {

                /** State version */
                version?: (string|null);

                /** State settingsSchemaJson */
                settingsSchemaJson?: (string|null);

                /** State properties */
                properties?: (eyepiece.rpc.v1.IProperty[]|null);

                /** State phase */
                phase?: (string|null);

                /** State desiredRunning */
                desiredRunning?: (boolean|null);

                /** State connected */
                connected?: (boolean|null);

                /** State commandLine */
                commandLine?: (string|null);

                /** State runningCommandLine */
                runningCommandLine?: (string|null);

                /** State runningProperties */
                runningProperties?: (eyepiece.rpc.v1.IProperty[]|null);
            }

            /** Represents a State. */
            class State implements IState {

                /**
                 * Constructs a new State.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IState);

                /** State version. */
                public version: string;

                /** State settingsSchemaJson. */
                public settingsSchemaJson: string;

                /** State properties. */
                public properties: eyepiece.rpc.v1.IProperty[];

                /** State phase. */
                public phase: string;

                /** State desiredRunning. */
                public desiredRunning: boolean;

                /** State connected. */
                public connected: boolean;

                /** State commandLine. */
                public commandLine: string;

                /** State runningCommandLine. */
                public runningCommandLine: string;

                /** State runningProperties. */
                public runningProperties: eyepiece.rpc.v1.IProperty[];

                /**
                 * Creates a new State instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns State instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IState): eyepiece.rpc.v1.State;

                /**
                 * Encodes the specified State message. Does not implicitly {@link eyepiece.rpc.v1.State.verify|verify} messages.
                 * @param message State message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IState, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified State message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.State.verify|verify} messages.
                 * @param message State message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IState, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a State message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns State
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.State;

                /**
                 * Decodes a State message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns State
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.State;

                /**
                 * Verifies a State message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a State message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns State
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.State;

                /**
                 * Creates a plain object from a State message. Also converts values to other types if specified.
                 * @param message State
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.State, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this State to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for State
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of an Error. */
            interface IError {

                /** Error message */
                message?: (string|null);
            }

            /** Represents an Error. */
            class Error implements IError {

                /**
                 * Constructs a new Error.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IError);

                /** Error message. */
                public message: string;

                /**
                 * Creates a new Error instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Error instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IError): eyepiece.rpc.v1.Error;

                /**
                 * Encodes the specified Error message. Does not implicitly {@link eyepiece.rpc.v1.Error.verify|verify} messages.
                 * @param message Error message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IError, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Error message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Error.verify|verify} messages.
                 * @param message Error message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IError, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes an Error message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Error
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.Error;

                /**
                 * Decodes an Error message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Error
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.Error;

                /**
                 * Verifies an Error message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates an Error message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Error
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.Error;

                /**
                 * Creates a plain object from an Error message. Also converts values to other types if specified.
                 * @param message Error
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.Error, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Error to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for Error
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a Log. */
            interface ILog {

                /** Log line */
                line?: (string|null);
            }

            /** Represents a Log. */
            class Log implements ILog {

                /**
                 * Constructs a new Log.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.ILog);

                /** Log line. */
                public line: string;

                /**
                 * Creates a new Log instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Log instance
                 */
                public static create(properties?: eyepiece.rpc.v1.ILog): eyepiece.rpc.v1.Log;

                /**
                 * Encodes the specified Log message. Does not implicitly {@link eyepiece.rpc.v1.Log.verify|verify} messages.
                 * @param message Log message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.ILog, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Log message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Log.verify|verify} messages.
                 * @param message Log message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.ILog, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Log message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Log
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.Log;

                /**
                 * Decodes a Log message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Log
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.Log;

                /**
                 * Verifies a Log message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Log message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Log
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.Log;

                /**
                 * Creates a plain object from a Log message. Also converts values to other types if specified.
                 * @param message Log
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.Log, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Log to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for Log
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a Pose. */
            interface IPose {

                /** Pose yaw */
                yaw?: (number|null);

                /** Pose pitch */
                pitch?: (number|null);

                /** Pose roll */
                roll?: (number|null);

                /** Pose deviceUs */
                deviceUs?: (number|Long|null);

                /** Pose active */
                active?: (boolean|null);

                /** Pose source */
                source?: (number|null);
            }

            /** Represents a Pose. */
            class Pose implements IPose {

                /**
                 * Constructs a new Pose.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IPose);

                /** Pose yaw. */
                public yaw: number;

                /** Pose pitch. */
                public pitch: number;

                /** Pose roll. */
                public roll: number;

                /** Pose deviceUs. */
                public deviceUs: (number|Long);

                /** Pose active. */
                public active: boolean;

                /** Pose source. */
                public source: number;

                /**
                 * Creates a new Pose instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Pose instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IPose): eyepiece.rpc.v1.Pose;

                /**
                 * Encodes the specified Pose message. Does not implicitly {@link eyepiece.rpc.v1.Pose.verify|verify} messages.
                 * @param message Pose message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IPose, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Pose message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Pose.verify|verify} messages.
                 * @param message Pose message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IPose, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Pose message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Pose
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.Pose;

                /**
                 * Decodes a Pose message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Pose
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.Pose;

                /**
                 * Verifies a Pose message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Pose message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Pose
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.Pose;

                /**
                 * Creates a plain object from a Pose message. Also converts values to other types if specified.
                 * @param message Pose
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.Pose, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Pose to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for Pose
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a Metrics. */
            interface IMetrics {

                /** Metrics intervalS */
                intervalS?: (number|null);

                /** Metrics sentFps */
                sentFps?: (number|null);

                /** Metrics mbps */
                mbps?: (number|null);

                /** Metrics shownFps */
                shownFps?: (number|null);

                /** Metrics latP50Ms */
                latP50Ms?: (number|null);

                /** Metrics latP95Ms */
                latP95Ms?: (number|null);

                /** Metrics capSendP50Ms */
                capSendP50Ms?: (number|null);

                /** Metrics capSendP95Ms */
                capSendP95Ms?: (number|null);

                /** Metrics capEncP50Ms */
                capEncP50Ms?: (number|null);

                /** Metrics capEncP95Ms */
                capEncP95Ms?: (number|null);

                /** Metrics encP50Ms */
                encP50Ms?: (number|null);

                /** Metrics encP95Ms */
                encP95Ms?: (number|null);

                /** Metrics decP50Ms */
                decP50Ms?: (number|null);

                /** Metrics decP95Ms */
                decP95Ms?: (number|null);

                /** Metrics cvtP50Ms */
                cvtP50Ms?: (number|null);

                /** Metrics cvtP95Ms */
                cvtP95Ms?: (number|null);

                /** Metrics recvP50Ms */
                recvP50Ms?: (number|null);

                /** Metrics recvP95Ms */
                recvP95Ms?: (number|null);

                /** Metrics qMax */
                qMax?: (number|null);

                /** Metrics dropped */
                dropped?: (number|null);

                /** Metrics skipped */
                skipped?: (number|null);

                /** Metrics idr */
                idr?: (number|null);

                /** Metrics capturedFps */
                capturedFps?: (number|null);

                /** Metrics poseHz */
                poseHz?: (number|null);

                /** Metrics rttMs */
                rttMs?: (number|null);

                /** Metrics reportIntervalS */
                reportIntervalS?: (number|null);

                /** Metrics sampledAtMs */
                sampledAtMs?: (number|null);
            }

            /** Represents a Metrics. */
            class Metrics implements IMetrics {

                /**
                 * Constructs a new Metrics.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IMetrics);

                /** Metrics intervalS. */
                public intervalS: number;

                /** Metrics sentFps. */
                public sentFps: number;

                /** Metrics mbps. */
                public mbps: number;

                /** Metrics shownFps. */
                public shownFps: number;

                /** Metrics latP50Ms. */
                public latP50Ms: number;

                /** Metrics latP95Ms. */
                public latP95Ms: number;

                /** Metrics capSendP50Ms. */
                public capSendP50Ms: number;

                /** Metrics capSendP95Ms. */
                public capSendP95Ms: number;

                /** Metrics capEncP50Ms. */
                public capEncP50Ms: number;

                /** Metrics capEncP95Ms. */
                public capEncP95Ms: number;

                /** Metrics encP50Ms. */
                public encP50Ms: number;

                /** Metrics encP95Ms. */
                public encP95Ms: number;

                /** Metrics decP50Ms. */
                public decP50Ms: number;

                /** Metrics decP95Ms. */
                public decP95Ms: number;

                /** Metrics cvtP50Ms. */
                public cvtP50Ms: number;

                /** Metrics cvtP95Ms. */
                public cvtP95Ms: number;

                /** Metrics recvP50Ms. */
                public recvP50Ms: number;

                /** Metrics recvP95Ms. */
                public recvP95Ms: number;

                /** Metrics qMax. */
                public qMax: number;

                /** Metrics dropped. */
                public dropped: number;

                /** Metrics skipped. */
                public skipped: number;

                /** Metrics idr. */
                public idr: number;

                /** Metrics capturedFps. */
                public capturedFps: number;

                /** Metrics poseHz. */
                public poseHz: number;

                /** Metrics rttMs. */
                public rttMs: number;

                /** Metrics reportIntervalS. */
                public reportIntervalS: number;

                /** Metrics sampledAtMs. */
                public sampledAtMs: number;

                /**
                 * Creates a new Metrics instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Metrics instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IMetrics): eyepiece.rpc.v1.Metrics;

                /**
                 * Encodes the specified Metrics message. Does not implicitly {@link eyepiece.rpc.v1.Metrics.verify|verify} messages.
                 * @param message Metrics message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IMetrics, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Metrics message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Metrics.verify|verify} messages.
                 * @param message Metrics message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IMetrics, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Metrics message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Metrics
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.Metrics;

                /**
                 * Decodes a Metrics message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Metrics
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.Metrics;

                /**
                 * Verifies a Metrics message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Metrics message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Metrics
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.Metrics;

                /**
                 * Creates a plain object from a Metrics message. Also converts values to other types if specified.
                 * @param message Metrics
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.Metrics, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Metrics to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for Metrics
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a Preview. */
            interface IPreview {

                /** Preview accessUnit */
                accessUnit?: (Uint8Array|null);

                /** Preview key */
                key?: (boolean|null);

                /** Preview codec */
                codec?: (string|null);

                /** Preview timestampUs */
                timestampUs?: (number|Long|null);

                /** Preview discontinuity */
                discontinuity?: (boolean|null);

                /** Preview packedAlpha */
                packedAlpha?: (boolean|null);
            }

            /** Represents a Preview. */
            class Preview implements IPreview {

                /**
                 * Constructs a new Preview.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IPreview);

                /** Preview accessUnit. */
                public accessUnit: Uint8Array;

                /** Preview key. */
                public key: boolean;

                /** Preview codec. */
                public codec: string;

                /** Preview timestampUs. */
                public timestampUs: (number|Long);

                /** Preview discontinuity. */
                public discontinuity: boolean;

                /** Preview packedAlpha. */
                public packedAlpha: boolean;

                /**
                 * Creates a new Preview instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Preview instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IPreview): eyepiece.rpc.v1.Preview;

                /**
                 * Encodes the specified Preview message. Does not implicitly {@link eyepiece.rpc.v1.Preview.verify|verify} messages.
                 * @param message Preview message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IPreview, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Preview message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Preview.verify|verify} messages.
                 * @param message Preview message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IPreview, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Preview message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Preview
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.Preview;

                /**
                 * Decodes a Preview message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Preview
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.Preview;

                /**
                 * Verifies a Preview message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Preview message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Preview
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.Preview;

                /**
                 * Creates a plain object from a Preview message. Also converts values to other types if specified.
                 * @param message Preview
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.Preview, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Preview to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for Preview
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a KmsOutputs. */
            interface IKmsOutputs {

                /** KmsOutputs outputs */
                outputs?: (string[]|null);
            }

            /** Represents a KmsOutputs. */
            class KmsOutputs implements IKmsOutputs {

                /**
                 * Constructs a new KmsOutputs.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IKmsOutputs);

                /** KmsOutputs outputs. */
                public outputs: string[];

                /**
                 * Creates a new KmsOutputs instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns KmsOutputs instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IKmsOutputs): eyepiece.rpc.v1.KmsOutputs;

                /**
                 * Encodes the specified KmsOutputs message. Does not implicitly {@link eyepiece.rpc.v1.KmsOutputs.verify|verify} messages.
                 * @param message KmsOutputs message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IKmsOutputs, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified KmsOutputs message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.KmsOutputs.verify|verify} messages.
                 * @param message KmsOutputs message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IKmsOutputs, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a KmsOutputs message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns KmsOutputs
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.KmsOutputs;

                /**
                 * Decodes a KmsOutputs message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns KmsOutputs
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.KmsOutputs;

                /**
                 * Verifies a KmsOutputs message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a KmsOutputs message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns KmsOutputs
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.KmsOutputs;

                /**
                 * Creates a plain object from a KmsOutputs message. Also converts values to other types if specified.
                 * @param message KmsOutputs
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.KmsOutputs, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this KmsOutputs to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for KmsOutputs
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a Video. */
            interface IVideo {

                /** Video ptsUs */
                ptsUs?: (number|Long|null);

                /** Video codec */
                codec?: (number|null);

                /** Video key */
                key?: (boolean|null);

                /** Video accessUnit */
                accessUnit?: (Uint8Array|null);
            }

            /** Represents a Video. */
            class Video implements IVideo {

                /**
                 * Constructs a new Video.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IVideo);

                /** Video ptsUs. */
                public ptsUs: (number|Long);

                /** Video codec. */
                public codec: number;

                /** Video key. */
                public key: boolean;

                /** Video accessUnit. */
                public accessUnit: Uint8Array;

                /**
                 * Creates a new Video instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Video instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IVideo): eyepiece.rpc.v1.Video;

                /**
                 * Encodes the specified Video message. Does not implicitly {@link eyepiece.rpc.v1.Video.verify|verify} messages.
                 * @param message Video message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IVideo, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Video message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Video.verify|verify} messages.
                 * @param message Video message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IVideo, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Video message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Video
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.Video;

                /**
                 * Decodes a Video message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Video
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.Video;

                /**
                 * Verifies a Video message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Video message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Video
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.Video;

                /**
                 * Creates a plain object from a Video message. Also converts values to other types if specified.
                 * @param message Video
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.Video, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Video to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for Video
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a Ping. */
            interface IPing {

                /** Ping pcUs */
                pcUs?: (number|Long|null);
            }

            /** Represents a Ping. */
            class Ping implements IPing {

                /**
                 * Constructs a new Ping.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IPing);

                /** Ping pcUs. */
                public pcUs: (number|Long);

                /**
                 * Creates a new Ping instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Ping instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IPing): eyepiece.rpc.v1.Ping;

                /**
                 * Encodes the specified Ping message. Does not implicitly {@link eyepiece.rpc.v1.Ping.verify|verify} messages.
                 * @param message Ping message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IPing, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Ping message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Ping.verify|verify} messages.
                 * @param message Ping message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IPing, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Ping message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Ping
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.Ping;

                /**
                 * Decodes a Ping message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Ping
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.Ping;

                /**
                 * Verifies a Ping message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Ping message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Ping
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.Ping;

                /**
                 * Creates a plain object from a Ping message. Also converts values to other types if specified.
                 * @param message Ping
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.Ping, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Ping to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for Ping
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a Control. */
            interface IControl {

                /** Control operation */
                operation?: (number|null);

                /** Control enabled */
                enabled?: (boolean|null);
            }

            /** Represents a Control. */
            class Control implements IControl {

                /**
                 * Constructs a new Control.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IControl);

                /** Control operation. */
                public operation: number;

                /** Control enabled. */
                public enabled: boolean;

                /**
                 * Creates a new Control instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Control instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IControl): eyepiece.rpc.v1.Control;

                /**
                 * Encodes the specified Control message. Does not implicitly {@link eyepiece.rpc.v1.Control.verify|verify} messages.
                 * @param message Control message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IControl, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Control message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Control.verify|verify} messages.
                 * @param message Control message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IControl, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Control message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Control
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.Control;

                /**
                 * Decodes a Control message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Control
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.Control;

                /**
                 * Verifies a Control message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Control message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Control
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.Control;

                /**
                 * Creates a plain object from a Control message. Also converts values to other types if specified.
                 * @param message Control
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.Control, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Control to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for Control
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a Pane. */
            interface IPane {

                /** Pane x */
                x?: (number|null);

                /** Pane y */
                y?: (number|null);

                /** Pane width */
                width?: (number|null);

                /** Pane height */
                height?: (number|null);

                /** Pane format */
                format?: (number|null);
            }

            /** Represents a Pane. */
            class Pane implements IPane {

                /**
                 * Constructs a new Pane.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IPane);

                /** Pane x. */
                public x: number;

                /** Pane y. */
                public y: number;

                /** Pane width. */
                public width: number;

                /** Pane height. */
                public height: number;

                /** Pane format. */
                public format: number;

                /**
                 * Creates a new Pane instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Pane instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IPane): eyepiece.rpc.v1.Pane;

                /**
                 * Encodes the specified Pane message. Does not implicitly {@link eyepiece.rpc.v1.Pane.verify|verify} messages.
                 * @param message Pane message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IPane, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Pane message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Pane.verify|verify} messages.
                 * @param message Pane message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IPane, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Pane message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Pane
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.Pane;

                /**
                 * Decodes a Pane message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Pane
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.Pane;

                /**
                 * Verifies a Pane message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Pane message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Pane
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.Pane;

                /**
                 * Creates a plain object from a Pane message. Also converts values to other types if specified.
                 * @param message Pane
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.Pane, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Pane to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for Pane
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a Configure. */
            interface IConfigure {

                /** Configure codec */
                codec?: (number|null);

                /** Configure layout */
                layout?: (number|null);

                /** Configure pane */
                pane?: (eyepiece.rpc.v1.IPane|null);
            }

            /** Represents a Configure. */
            class Configure implements IConfigure {

                /**
                 * Constructs a new Configure.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IConfigure);

                /** Configure codec. */
                public codec: number;

                /** Configure layout. */
                public layout: number;

                /** Configure pane. */
                public pane?: (eyepiece.rpc.v1.IPane|null);

                /**
                 * Creates a new Configure instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Configure instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IConfigure): eyepiece.rpc.v1.Configure;

                /**
                 * Encodes the specified Configure message. Does not implicitly {@link eyepiece.rpc.v1.Configure.verify|verify} messages.
                 * @param message Configure message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IConfigure, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Configure message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Configure.verify|verify} messages.
                 * @param message Configure message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IConfigure, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Configure message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Configure
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.Configure;

                /**
                 * Decodes a Configure message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Configure
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.Configure;

                /**
                 * Verifies a Configure message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Configure message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Configure
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.Configure;

                /**
                 * Creates a plain object from a Configure message. Also converts values to other types if specified.
                 * @param message Configure
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.Configure, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Configure to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for Configure
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a GuiAlpha. */
            interface IGuiAlpha {

                /** GuiAlpha alpha */
                alpha?: (number|null);
            }

            /** Represents a GuiAlpha. */
            class GuiAlpha implements IGuiAlpha {

                /**
                 * Constructs a new GuiAlpha.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IGuiAlpha);

                /** GuiAlpha alpha. */
                public alpha: number;

                /**
                 * Creates a new GuiAlpha instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns GuiAlpha instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IGuiAlpha): eyepiece.rpc.v1.GuiAlpha;

                /**
                 * Encodes the specified GuiAlpha message. Does not implicitly {@link eyepiece.rpc.v1.GuiAlpha.verify|verify} messages.
                 * @param message GuiAlpha message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IGuiAlpha, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified GuiAlpha message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.GuiAlpha.verify|verify} messages.
                 * @param message GuiAlpha message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IGuiAlpha, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a GuiAlpha message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns GuiAlpha
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.GuiAlpha;

                /**
                 * Decodes a GuiAlpha message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns GuiAlpha
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.GuiAlpha;

                /**
                 * Verifies a GuiAlpha message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a GuiAlpha message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns GuiAlpha
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.GuiAlpha;

                /**
                 * Creates a plain object from a GuiAlpha message. Also converts values to other types if specified.
                 * @param message GuiAlpha
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.GuiAlpha, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this GuiAlpha to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for GuiAlpha
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a PaneAlpha. */
            interface IPaneAlpha {

                /** PaneAlpha alpha */
                alpha?: (number|null);
            }

            /** Represents a PaneAlpha. */
            class PaneAlpha implements IPaneAlpha {

                /**
                 * Constructs a new PaneAlpha.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IPaneAlpha);

                /** PaneAlpha alpha. */
                public alpha: number;

                /**
                 * Creates a new PaneAlpha instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns PaneAlpha instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IPaneAlpha): eyepiece.rpc.v1.PaneAlpha;

                /**
                 * Encodes the specified PaneAlpha message. Does not implicitly {@link eyepiece.rpc.v1.PaneAlpha.verify|verify} messages.
                 * @param message PaneAlpha message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IPaneAlpha, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified PaneAlpha message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.PaneAlpha.verify|verify} messages.
                 * @param message PaneAlpha message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IPaneAlpha, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a PaneAlpha message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns PaneAlpha
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.PaneAlpha;

                /**
                 * Decodes a PaneAlpha message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns PaneAlpha
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.PaneAlpha;

                /**
                 * Verifies a PaneAlpha message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a PaneAlpha message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns PaneAlpha
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.PaneAlpha;

                /**
                 * Creates a plain object from a PaneAlpha message. Also converts values to other types if specified.
                 * @param message PaneAlpha
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.PaneAlpha, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this PaneAlpha to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for PaneAlpha
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a Pong. */
            interface IPong {

                /** Pong pcUs */
                pcUs?: (number|Long|null);

                /** Pong deviceUs */
                deviceUs?: (number|Long|null);
            }

            /** Represents a Pong. */
            class Pong implements IPong {

                /**
                 * Constructs a new Pong.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IPong);

                /** Pong pcUs. */
                public pcUs: (number|Long);

                /** Pong deviceUs. */
                public deviceUs: (number|Long);

                /**
                 * Creates a new Pong instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Pong instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IPong): eyepiece.rpc.v1.Pong;

                /**
                 * Encodes the specified Pong message. Does not implicitly {@link eyepiece.rpc.v1.Pong.verify|verify} messages.
                 * @param message Pong message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IPong, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Pong message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Pong.verify|verify} messages.
                 * @param message Pong message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IPong, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Pong message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Pong
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.Pong;

                /**
                 * Decodes a Pong message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Pong
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.Pong;

                /**
                 * Verifies a Pong message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Pong message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Pong
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.Pong;

                /**
                 * Creates a plain object from a Pong message. Also converts values to other types if specified.
                 * @param message Pong
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.Pong, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Pong to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for Pong
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a Frame. */
            interface IFrame {

                /** Frame ptsUs */
                ptsUs?: (number|Long|null);

                /** Frame receivedUs */
                receivedUs?: (number|Long|null);

                /** Frame shownUs */
                shownUs?: (number|Long|null);

                /** Frame decodeUs */
                decodeUs?: (number|null);

                /** Frame convertUs */
                convertUs?: (number|null);

                /** Frame queueDepth */
                queueDepth?: (number|null);

                /** Frame dropped */
                dropped?: (number|null);
            }

            /** Represents a Frame. */
            class Frame implements IFrame {

                /**
                 * Constructs a new Frame.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IFrame);

                /** Frame ptsUs. */
                public ptsUs: (number|Long);

                /** Frame receivedUs. */
                public receivedUs: (number|Long);

                /** Frame shownUs. */
                public shownUs: (number|Long);

                /** Frame decodeUs. */
                public decodeUs: number;

                /** Frame convertUs. */
                public convertUs: number;

                /** Frame queueDepth. */
                public queueDepth: number;

                /** Frame dropped. */
                public dropped: number;

                /**
                 * Creates a new Frame instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Frame instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IFrame): eyepiece.rpc.v1.Frame;

                /**
                 * Encodes the specified Frame message. Does not implicitly {@link eyepiece.rpc.v1.Frame.verify|verify} messages.
                 * @param message Frame message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IFrame, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Frame message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Frame.verify|verify} messages.
                 * @param message Frame message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IFrame, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Frame message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Frame
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.Frame;

                /**
                 * Decodes a Frame message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Frame
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.Frame;

                /**
                 * Verifies a Frame message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Frame message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Frame
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.Frame;

                /**
                 * Creates a plain object from a Frame message. Also converts values to other types if specified.
                 * @param message Frame
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.Frame, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Frame to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for Frame
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of an Info. */
            interface IInfo {

                /** Info version */
                version?: (number|null);

                /** Info framebufferWidth */
                framebufferWidth?: (number|null);

                /** Info framebufferHeight */
                framebufferHeight?: (number|null);

                /** Info decoder */
                decoder?: (number|null);

                /** Info headtrack */
                headtrack?: (number|null);

                /** Info flags */
                flags?: (number|null);
            }

            /** Represents an Info. */
            class Info implements IInfo {

                /**
                 * Constructs a new Info.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IInfo);

                /** Info version. */
                public version: number;

                /** Info framebufferWidth. */
                public framebufferWidth: number;

                /** Info framebufferHeight. */
                public framebufferHeight: number;

                /** Info decoder. */
                public decoder: number;

                /** Info headtrack. */
                public headtrack: number;

                /** Info flags. */
                public flags: number;

                /**
                 * Creates a new Info instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Info instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IInfo): eyepiece.rpc.v1.Info;

                /**
                 * Encodes the specified Info message. Does not implicitly {@link eyepiece.rpc.v1.Info.verify|verify} messages.
                 * @param message Info message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IInfo, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Info message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Info.verify|verify} messages.
                 * @param message Info message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IInfo, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes an Info message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Info
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.Info;

                /**
                 * Decodes an Info message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Info
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.Info;

                /**
                 * Verifies an Info message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates an Info message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Info
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.Info;

                /**
                 * Creates a plain object from an Info message. Also converts values to other types if specified.
                 * @param message Info
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.Info, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Info to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for Info
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of a KeyRequest. */
            interface IKeyRequest {

                /** KeyRequest reason */
                reason?: (number|null);
            }

            /** Represents a KeyRequest. */
            class KeyRequest implements IKeyRequest {

                /**
                 * Constructs a new KeyRequest.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IKeyRequest);

                /** KeyRequest reason. */
                public reason: number;

                /**
                 * Creates a new KeyRequest instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns KeyRequest instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IKeyRequest): eyepiece.rpc.v1.KeyRequest;

                /**
                 * Encodes the specified KeyRequest message. Does not implicitly {@link eyepiece.rpc.v1.KeyRequest.verify|verify} messages.
                 * @param message KeyRequest message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IKeyRequest, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified KeyRequest message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.KeyRequest.verify|verify} messages.
                 * @param message KeyRequest message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IKeyRequest, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a KeyRequest message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns KeyRequest
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.KeyRequest;

                /**
                 * Decodes a KeyRequest message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns KeyRequest
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.KeyRequest;

                /**
                 * Verifies a KeyRequest message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a KeyRequest message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns KeyRequest
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.KeyRequest;

                /**
                 * Creates a plain object from a KeyRequest message. Also converts values to other types if specified.
                 * @param message KeyRequest
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.KeyRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this KeyRequest to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for KeyRequest
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }

            /** Properties of an Acknowledgement. */
            interface IAcknowledgement {

                /** Acknowledgement status */
                status?: (number|null);
            }

            /** Represents an Acknowledgement. */
            class Acknowledgement implements IAcknowledgement {

                /**
                 * Constructs a new Acknowledgement.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: eyepiece.rpc.v1.IAcknowledgement);

                /** Acknowledgement status. */
                public status: number;

                /**
                 * Creates a new Acknowledgement instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Acknowledgement instance
                 */
                public static create(properties?: eyepiece.rpc.v1.IAcknowledgement): eyepiece.rpc.v1.Acknowledgement;

                /**
                 * Encodes the specified Acknowledgement message. Does not implicitly {@link eyepiece.rpc.v1.Acknowledgement.verify|verify} messages.
                 * @param message Acknowledgement message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: eyepiece.rpc.v1.IAcknowledgement, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Acknowledgement message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Acknowledgement.verify|verify} messages.
                 * @param message Acknowledgement message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: eyepiece.rpc.v1.IAcknowledgement, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes an Acknowledgement message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Acknowledgement
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): eyepiece.rpc.v1.Acknowledgement;

                /**
                 * Decodes an Acknowledgement message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Acknowledgement
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): eyepiece.rpc.v1.Acknowledgement;

                /**
                 * Verifies an Acknowledgement message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates an Acknowledgement message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Acknowledgement
                 */
                public static fromObject(object: { [k: string]: any }): eyepiece.rpc.v1.Acknowledgement;

                /**
                 * Creates a plain object from an Acknowledgement message. Also converts values to other types if specified.
                 * @param message Acknowledgement
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: eyepiece.rpc.v1.Acknowledgement, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Acknowledgement to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };

                /**
                 * Gets the default type url for Acknowledgement
                 * @param [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns The default type url
                 */
                public static getTypeUrl(typeUrlPrefix?: string): string;
            }
        }
    }
}
