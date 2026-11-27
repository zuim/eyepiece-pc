/*eslint-disable block-scoped-var, id-length, no-control-regex, no-magic-numbers, no-prototype-builtins, no-redeclare, no-shadow, no-var, sort-vars*/
import * as $protobuf from "protobufjs/minimal";

// Common aliases
const $Reader = $protobuf.Reader, $Writer = $protobuf.Writer, $util = $protobuf.util;

// Exported root namespace
const $root = $protobuf.roots["default"] || ($protobuf.roots["default"] = {});

export const eyepiece = $root.eyepiece = (() => {

    /**
     * Namespace eyepiece.
     * @exports eyepiece
     * @namespace
     */
    const eyepiece = {};

    eyepiece.rpc = (function() {

        /**
         * Namespace rpc.
         * @memberof eyepiece
         * @namespace
         */
        const rpc = {};

        rpc.v1 = (function() {

            /**
             * Namespace v1.
             * @memberof eyepiece.rpc
             * @namespace
             */
            const v1 = {};

            /**
             * Codec enum.
             * @name eyepiece.rpc.v1.Codec
             * @enum {number}
             * @property {number} CODEC_H264=0 CODEC_H264 value
             * @property {number} CODEC_HEVC=1 CODEC_HEVC value
             */
            v1.Codec = (function() {
                const valuesById = {}, values = Object.create(valuesById);
                values[valuesById[0] = "CODEC_H264"] = 0;
                values[valuesById[1] = "CODEC_HEVC"] = 1;
                return values;
            })();

            /**
             * ControlOperation enum.
             * @name eyepiece.rpc.v1.ControlOperation
             * @enum {number}
             * @property {number} CONTROL_UNSPECIFIED=0 CONTROL_UNSPECIFIED value
             * @property {number} CONTROL_RECENTER=1 CONTROL_RECENTER value
             * @property {number} CONTROL_QUIT=2 CONTROL_QUIT value
             * @property {number} CONTROL_HEADTRACK=3 CONTROL_HEADTRACK value
             */
            v1.ControlOperation = (function() {
                const valuesById = {}, values = Object.create(valuesById);
                values[valuesById[0] = "CONTROL_UNSPECIFIED"] = 0;
                values[valuesById[1] = "CONTROL_RECENTER"] = 1;
                values[valuesById[2] = "CONTROL_QUIT"] = 2;
                values[valuesById[3] = "CONTROL_HEADTRACK"] = 3;
                return values;
            })();

            /**
             * StreamLayout enum.
             * @name eyepiece.rpc.v1.StreamLayout
             * @enum {number}
             * @property {number} LAYOUT_MONO=0 LAYOUT_MONO value
             * @property {number} LAYOUT_SBS=1 LAYOUT_SBS value
             */
            v1.StreamLayout = (function() {
                const valuesById = {}, values = Object.create(valuesById);
                values[valuesById[0] = "LAYOUT_MONO"] = 0;
                values[valuesById[1] = "LAYOUT_SBS"] = 1;
                return values;
            })();

            /**
             * PaneFormat enum.
             * @name eyepiece.rpc.v1.PaneFormat
             * @enum {number}
             * @property {number} PANE_FORMAT_UNSPECIFIED=0 PANE_FORMAT_UNSPECIFIED value
             * @property {number} PANE_BGRA=1 PANE_BGRA value
             * @property {number} PANE_OPAQUE_BGRA=2 PANE_OPAQUE_BGRA value
             */
            v1.PaneFormat = (function() {
                const valuesById = {}, values = Object.create(valuesById);
                values[valuesById[0] = "PANE_FORMAT_UNSPECIFIED"] = 0;
                values[valuesById[1] = "PANE_BGRA"] = 1;
                values[valuesById[2] = "PANE_OPAQUE_BGRA"] = 2;
                return values;
            })();

            /**
             * PoseSource enum.
             * @name eyepiece.rpc.v1.PoseSource
             * @enum {number}
             * @property {number} POSE_SOURCE_UNSPECIFIED=0 POSE_SOURCE_UNSPECIFIED value
             * @property {number} POSE_SOURCE_MCU=3 POSE_SOURCE_MCU value
             */
            v1.PoseSource = (function() {
                const valuesById = {}, values = Object.create(valuesById);
                values[valuesById[0] = "POSE_SOURCE_UNSPECIFIED"] = 0;
                values[valuesById[3] = "POSE_SOURCE_MCU"] = 3;
                return values;
            })();

            /**
             * BridgeCapability enum.
             * @name eyepiece.rpc.v1.BridgeCapability
             * @enum {number}
             * @property {number} CAPABILITY_NONE=0 CAPABILITY_NONE value
             * @property {number} CAPABILITY_STEREO=1 CAPABILITY_STEREO value
             * @property {number} CAPABILITY_PANE_BGRA=4 CAPABILITY_PANE_BGRA value
             * @property {number} CAPABILITY_PANE_OPAQUE_BGRA=8 CAPABILITY_PANE_OPAQUE_BGRA value
             * @property {number} CAPABILITY_RECONFIG=32 CAPABILITY_RECONFIG value
             * @property {number} CAPABILITY_GUI_ALPHA=64 CAPABILITY_GUI_ALPHA value
             * @property {number} CAPABILITY_PANE_ALPHA=256 CAPABILITY_PANE_ALPHA value
             */
            v1.BridgeCapability = (function() {
                const valuesById = {}, values = Object.create(valuesById);
                values[valuesById[0] = "CAPABILITY_NONE"] = 0;
                values[valuesById[1] = "CAPABILITY_STEREO"] = 1;
                values[valuesById[4] = "CAPABILITY_PANE_BGRA"] = 4;
                values[valuesById[8] = "CAPABILITY_PANE_OPAQUE_BGRA"] = 8;
                values[valuesById[32] = "CAPABILITY_RECONFIG"] = 32;
                values[valuesById[64] = "CAPABILITY_GUI_ALPHA"] = 64;
                values[valuesById[256] = "CAPABILITY_PANE_ALPHA"] = 256;
                return values;
            })();

            v1.Envelope = (function() {

                /**
                 * Properties of an Envelope.
                 * @memberof eyepiece.rpc.v1
                 * @interface IEnvelope
                 * @property {string|null} [id] Envelope id
                 * @property {string|null} [sender] Envelope sender
                 * @property {string|null} [recipient] Envelope recipient
                 * @property {eyepiece.rpc.v1.IGetState|null} [getState] Envelope getState
                 * @property {eyepiece.rpc.v1.ISetProperties|null} [setProperties] Envelope setProperties
                 * @property {eyepiece.rpc.v1.IStart|null} [start] Envelope start
                 * @property {eyepiece.rpc.v1.IStop|null} [stop] Envelope stop
                 * @property {eyepiece.rpc.v1.IRestart|null} [restart] Envelope restart
                 * @property {eyepiece.rpc.v1.IRecenter|null} [recenter] Envelope recenter
                 * @property {eyepiece.rpc.v1.IForceIdr|null} [forceIdr] Envelope forceIdr
                 * @property {eyepiece.rpc.v1.ISetPreview|null} [setPreview] Envelope setPreview
                 * @property {eyepiece.rpc.v1.ISetCommandLine|null} [setCommandLine] Envelope setCommandLine
                 * @property {eyepiece.rpc.v1.IListKmsOutputs|null} [listKmsOutputs] Envelope listKmsOutputs
                 * @property {eyepiece.rpc.v1.IState|null} [state] Envelope state
                 * @property {eyepiece.rpc.v1.IError|null} [error] Envelope error
                 * @property {eyepiece.rpc.v1.ILog|null} [log] Envelope log
                 * @property {eyepiece.rpc.v1.IPose|null} [pose] Envelope pose
                 * @property {eyepiece.rpc.v1.IMetrics|null} [metrics] Envelope metrics
                 * @property {eyepiece.rpc.v1.IPreview|null} [preview] Envelope preview
                 * @property {boolean|null} [acknowledged] Envelope acknowledged
                 * @property {eyepiece.rpc.v1.IKmsOutputs|null} [kmsOutputs] Envelope kmsOutputs
                 * @property {eyepiece.rpc.v1.IVideo|null} [video] Envelope video
                 * @property {eyepiece.rpc.v1.IPing|null} [ping] Envelope ping
                 * @property {eyepiece.rpc.v1.IControl|null} [control] Envelope control
                 * @property {eyepiece.rpc.v1.IGuiAlpha|null} [guiAlpha] Envelope guiAlpha
                 * @property {eyepiece.rpc.v1.IPong|null} [pong] Envelope pong
                 * @property {eyepiece.rpc.v1.IFrame|null} [frame] Envelope frame
                 * @property {eyepiece.rpc.v1.IInfo|null} [info] Envelope info
                 * @property {eyepiece.rpc.v1.IKeyRequest|null} [keyRequest] Envelope keyRequest
                 * @property {eyepiece.rpc.v1.IAcknowledgement|null} [guiAlphaAck] Envelope guiAlphaAck
                 * @property {eyepiece.rpc.v1.IPaneAlpha|null} [paneAlpha] Envelope paneAlpha
                 * @property {eyepiece.rpc.v1.IAcknowledgement|null} [paneAlphaAck] Envelope paneAlphaAck
                 * @property {eyepiece.rpc.v1.IConfigure|null} [configure] Envelope configure
                 * @property {eyepiece.rpc.v1.IAcknowledgement|null} [configureAck] Envelope configureAck
                 */

                /**
                 * Constructs a new Envelope.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents an Envelope.
                 * @implements IEnvelope
                 * @constructor
                 * @param {eyepiece.rpc.v1.IEnvelope=} [properties] Properties to set
                 */
                function Envelope(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Envelope id.
                 * @member {string|null|undefined} id
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.id = null;

                /**
                 * Envelope sender.
                 * @member {string|null|undefined} sender
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.sender = null;

                /**
                 * Envelope recipient.
                 * @member {string|null|undefined} recipient
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.recipient = null;

                /**
                 * Envelope getState.
                 * @member {eyepiece.rpc.v1.IGetState|null|undefined} getState
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.getState = null;

                /**
                 * Envelope setProperties.
                 * @member {eyepiece.rpc.v1.ISetProperties|null|undefined} setProperties
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.setProperties = null;

                /**
                 * Envelope start.
                 * @member {eyepiece.rpc.v1.IStart|null|undefined} start
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.start = null;

                /**
                 * Envelope stop.
                 * @member {eyepiece.rpc.v1.IStop|null|undefined} stop
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.stop = null;

                /**
                 * Envelope restart.
                 * @member {eyepiece.rpc.v1.IRestart|null|undefined} restart
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.restart = null;

                /**
                 * Envelope recenter.
                 * @member {eyepiece.rpc.v1.IRecenter|null|undefined} recenter
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.recenter = null;

                /**
                 * Envelope forceIdr.
                 * @member {eyepiece.rpc.v1.IForceIdr|null|undefined} forceIdr
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.forceIdr = null;

                /**
                 * Envelope setPreview.
                 * @member {eyepiece.rpc.v1.ISetPreview|null|undefined} setPreview
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.setPreview = null;

                /**
                 * Envelope setCommandLine.
                 * @member {eyepiece.rpc.v1.ISetCommandLine|null|undefined} setCommandLine
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.setCommandLine = null;

                /**
                 * Envelope listKmsOutputs.
                 * @member {eyepiece.rpc.v1.IListKmsOutputs|null|undefined} listKmsOutputs
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.listKmsOutputs = null;

                /**
                 * Envelope state.
                 * @member {eyepiece.rpc.v1.IState|null|undefined} state
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.state = null;

                /**
                 * Envelope error.
                 * @member {eyepiece.rpc.v1.IError|null|undefined} error
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.error = null;

                /**
                 * Envelope log.
                 * @member {eyepiece.rpc.v1.ILog|null|undefined} log
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.log = null;

                /**
                 * Envelope pose.
                 * @member {eyepiece.rpc.v1.IPose|null|undefined} pose
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.pose = null;

                /**
                 * Envelope metrics.
                 * @member {eyepiece.rpc.v1.IMetrics|null|undefined} metrics
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.metrics = null;

                /**
                 * Envelope preview.
                 * @member {eyepiece.rpc.v1.IPreview|null|undefined} preview
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.preview = null;

                /**
                 * Envelope acknowledged.
                 * @member {boolean|null|undefined} acknowledged
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.acknowledged = null;

                /**
                 * Envelope kmsOutputs.
                 * @member {eyepiece.rpc.v1.IKmsOutputs|null|undefined} kmsOutputs
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.kmsOutputs = null;

                /**
                 * Envelope video.
                 * @member {eyepiece.rpc.v1.IVideo|null|undefined} video
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.video = null;

                /**
                 * Envelope ping.
                 * @member {eyepiece.rpc.v1.IPing|null|undefined} ping
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.ping = null;

                /**
                 * Envelope control.
                 * @member {eyepiece.rpc.v1.IControl|null|undefined} control
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.control = null;

                /**
                 * Envelope guiAlpha.
                 * @member {eyepiece.rpc.v1.IGuiAlpha|null|undefined} guiAlpha
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.guiAlpha = null;

                /**
                 * Envelope pong.
                 * @member {eyepiece.rpc.v1.IPong|null|undefined} pong
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.pong = null;

                /**
                 * Envelope frame.
                 * @member {eyepiece.rpc.v1.IFrame|null|undefined} frame
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.frame = null;

                /**
                 * Envelope info.
                 * @member {eyepiece.rpc.v1.IInfo|null|undefined} info
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.info = null;

                /**
                 * Envelope keyRequest.
                 * @member {eyepiece.rpc.v1.IKeyRequest|null|undefined} keyRequest
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.keyRequest = null;

                /**
                 * Envelope guiAlphaAck.
                 * @member {eyepiece.rpc.v1.IAcknowledgement|null|undefined} guiAlphaAck
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.guiAlphaAck = null;

                /**
                 * Envelope paneAlpha.
                 * @member {eyepiece.rpc.v1.IPaneAlpha|null|undefined} paneAlpha
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.paneAlpha = null;

                /**
                 * Envelope paneAlphaAck.
                 * @member {eyepiece.rpc.v1.IAcknowledgement|null|undefined} paneAlphaAck
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.paneAlphaAck = null;

                /**
                 * Envelope configure.
                 * @member {eyepiece.rpc.v1.IConfigure|null|undefined} configure
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.configure = null;

                /**
                 * Envelope configureAck.
                 * @member {eyepiece.rpc.v1.IAcknowledgement|null|undefined} configureAck
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Envelope.prototype.configureAck = null;

                // OneOf field names bound to virtual getters and setters
                let $oneOfFields;

                // Virtual OneOf for proto3 optional field
                Object.defineProperty(Envelope.prototype, "_id", {
                    get: $util.oneOfGetter($oneOfFields = ["id"]),
                    set: $util.oneOfSetter($oneOfFields)
                });

                // Virtual OneOf for proto3 optional field
                Object.defineProperty(Envelope.prototype, "_sender", {
                    get: $util.oneOfGetter($oneOfFields = ["sender"]),
                    set: $util.oneOfSetter($oneOfFields)
                });

                // Virtual OneOf for proto3 optional field
                Object.defineProperty(Envelope.prototype, "_recipient", {
                    get: $util.oneOfGetter($oneOfFields = ["recipient"]),
                    set: $util.oneOfSetter($oneOfFields)
                });

                /**
                 * Envelope message.
                 * @member {"getState"|"setProperties"|"start"|"stop"|"restart"|"recenter"|"forceIdr"|"setPreview"|"setCommandLine"|"listKmsOutputs"|"state"|"error"|"log"|"pose"|"metrics"|"preview"|"acknowledged"|"kmsOutputs"|"video"|"ping"|"control"|"guiAlpha"|"pong"|"frame"|"info"|"keyRequest"|"guiAlphaAck"|"paneAlpha"|"paneAlphaAck"|"configure"|"configureAck"|undefined} message
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 */
                Object.defineProperty(Envelope.prototype, "message", {
                    get: $util.oneOfGetter($oneOfFields = ["getState", "setProperties", "start", "stop", "restart", "recenter", "forceIdr", "setPreview", "setCommandLine", "listKmsOutputs", "state", "error", "log", "pose", "metrics", "preview", "acknowledged", "kmsOutputs", "video", "ping", "control", "guiAlpha", "pong", "frame", "info", "keyRequest", "guiAlphaAck", "paneAlpha", "paneAlphaAck", "configure", "configureAck"]),
                    set: $util.oneOfSetter($oneOfFields)
                });

                /**
                 * Creates a new Envelope instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @static
                 * @param {eyepiece.rpc.v1.IEnvelope=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.Envelope} Envelope instance
                 */
                Envelope.create = function create(properties) {
                    return new Envelope(properties);
                };

                /**
                 * Encodes the specified Envelope message. Does not implicitly {@link eyepiece.rpc.v1.Envelope.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @static
                 * @param {eyepiece.rpc.v1.IEnvelope} message Envelope message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Envelope.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.id != null && Object.hasOwnProperty.call(message, "id"))
                        writer.uint32(/* id 1, wireType 2 =*/10).string(message.id);
                    if (message.sender != null && Object.hasOwnProperty.call(message, "sender"))
                        writer.uint32(/* id 2, wireType 2 =*/18).string(message.sender);
                    if (message.recipient != null && Object.hasOwnProperty.call(message, "recipient"))
                        writer.uint32(/* id 3, wireType 2 =*/26).string(message.recipient);
                    if (message.getState != null && Object.hasOwnProperty.call(message, "getState"))
                        $root.eyepiece.rpc.v1.GetState.encode(message.getState, writer.uint32(/* id 10, wireType 2 =*/82).fork(), q + 1).ldelim();
                    if (message.setProperties != null && Object.hasOwnProperty.call(message, "setProperties"))
                        $root.eyepiece.rpc.v1.SetProperties.encode(message.setProperties, writer.uint32(/* id 11, wireType 2 =*/90).fork(), q + 1).ldelim();
                    if (message.start != null && Object.hasOwnProperty.call(message, "start"))
                        $root.eyepiece.rpc.v1.Start.encode(message.start, writer.uint32(/* id 12, wireType 2 =*/98).fork(), q + 1).ldelim();
                    if (message.stop != null && Object.hasOwnProperty.call(message, "stop"))
                        $root.eyepiece.rpc.v1.Stop.encode(message.stop, writer.uint32(/* id 13, wireType 2 =*/106).fork(), q + 1).ldelim();
                    if (message.restart != null && Object.hasOwnProperty.call(message, "restart"))
                        $root.eyepiece.rpc.v1.Restart.encode(message.restart, writer.uint32(/* id 14, wireType 2 =*/114).fork(), q + 1).ldelim();
                    if (message.recenter != null && Object.hasOwnProperty.call(message, "recenter"))
                        $root.eyepiece.rpc.v1.Recenter.encode(message.recenter, writer.uint32(/* id 15, wireType 2 =*/122).fork(), q + 1).ldelim();
                    if (message.forceIdr != null && Object.hasOwnProperty.call(message, "forceIdr"))
                        $root.eyepiece.rpc.v1.ForceIdr.encode(message.forceIdr, writer.uint32(/* id 16, wireType 2 =*/130).fork(), q + 1).ldelim();
                    if (message.setPreview != null && Object.hasOwnProperty.call(message, "setPreview"))
                        $root.eyepiece.rpc.v1.SetPreview.encode(message.setPreview, writer.uint32(/* id 17, wireType 2 =*/138).fork(), q + 1).ldelim();
                    if (message.setCommandLine != null && Object.hasOwnProperty.call(message, "setCommandLine"))
                        $root.eyepiece.rpc.v1.SetCommandLine.encode(message.setCommandLine, writer.uint32(/* id 18, wireType 2 =*/146).fork(), q + 1).ldelim();
                    if (message.listKmsOutputs != null && Object.hasOwnProperty.call(message, "listKmsOutputs"))
                        $root.eyepiece.rpc.v1.ListKmsOutputs.encode(message.listKmsOutputs, writer.uint32(/* id 19, wireType 2 =*/154).fork(), q + 1).ldelim();
                    if (message.state != null && Object.hasOwnProperty.call(message, "state"))
                        $root.eyepiece.rpc.v1.State.encode(message.state, writer.uint32(/* id 30, wireType 2 =*/242).fork(), q + 1).ldelim();
                    if (message.error != null && Object.hasOwnProperty.call(message, "error"))
                        $root.eyepiece.rpc.v1.Error.encode(message.error, writer.uint32(/* id 31, wireType 2 =*/250).fork(), q + 1).ldelim();
                    if (message.log != null && Object.hasOwnProperty.call(message, "log"))
                        $root.eyepiece.rpc.v1.Log.encode(message.log, writer.uint32(/* id 32, wireType 2 =*/258).fork(), q + 1).ldelim();
                    if (message.pose != null && Object.hasOwnProperty.call(message, "pose"))
                        $root.eyepiece.rpc.v1.Pose.encode(message.pose, writer.uint32(/* id 33, wireType 2 =*/266).fork(), q + 1).ldelim();
                    if (message.metrics != null && Object.hasOwnProperty.call(message, "metrics"))
                        $root.eyepiece.rpc.v1.Metrics.encode(message.metrics, writer.uint32(/* id 34, wireType 2 =*/274).fork(), q + 1).ldelim();
                    if (message.preview != null && Object.hasOwnProperty.call(message, "preview"))
                        $root.eyepiece.rpc.v1.Preview.encode(message.preview, writer.uint32(/* id 35, wireType 2 =*/282).fork(), q + 1).ldelim();
                    if (message.acknowledged != null && Object.hasOwnProperty.call(message, "acknowledged"))
                        writer.uint32(/* id 36, wireType 0 =*/288).bool(message.acknowledged);
                    if (message.kmsOutputs != null && Object.hasOwnProperty.call(message, "kmsOutputs"))
                        $root.eyepiece.rpc.v1.KmsOutputs.encode(message.kmsOutputs, writer.uint32(/* id 37, wireType 2 =*/298).fork(), q + 1).ldelim();
                    if (message.video != null && Object.hasOwnProperty.call(message, "video"))
                        $root.eyepiece.rpc.v1.Video.encode(message.video, writer.uint32(/* id 50, wireType 2 =*/402).fork(), q + 1).ldelim();
                    if (message.ping != null && Object.hasOwnProperty.call(message, "ping"))
                        $root.eyepiece.rpc.v1.Ping.encode(message.ping, writer.uint32(/* id 51, wireType 2 =*/410).fork(), q + 1).ldelim();
                    if (message.control != null && Object.hasOwnProperty.call(message, "control"))
                        $root.eyepiece.rpc.v1.Control.encode(message.control, writer.uint32(/* id 52, wireType 2 =*/418).fork(), q + 1).ldelim();
                    if (message.guiAlpha != null && Object.hasOwnProperty.call(message, "guiAlpha"))
                        $root.eyepiece.rpc.v1.GuiAlpha.encode(message.guiAlpha, writer.uint32(/* id 56, wireType 2 =*/450).fork(), q + 1).ldelim();
                    if (message.pong != null && Object.hasOwnProperty.call(message, "pong"))
                        $root.eyepiece.rpc.v1.Pong.encode(message.pong, writer.uint32(/* id 57, wireType 2 =*/458).fork(), q + 1).ldelim();
                    if (message.frame != null && Object.hasOwnProperty.call(message, "frame"))
                        $root.eyepiece.rpc.v1.Frame.encode(message.frame, writer.uint32(/* id 58, wireType 2 =*/466).fork(), q + 1).ldelim();
                    if (message.info != null && Object.hasOwnProperty.call(message, "info"))
                        $root.eyepiece.rpc.v1.Info.encode(message.info, writer.uint32(/* id 59, wireType 2 =*/474).fork(), q + 1).ldelim();
                    if (message.keyRequest != null && Object.hasOwnProperty.call(message, "keyRequest"))
                        $root.eyepiece.rpc.v1.KeyRequest.encode(message.keyRequest, writer.uint32(/* id 60, wireType 2 =*/482).fork(), q + 1).ldelim();
                    if (message.guiAlphaAck != null && Object.hasOwnProperty.call(message, "guiAlphaAck"))
                        $root.eyepiece.rpc.v1.Acknowledgement.encode(message.guiAlphaAck, writer.uint32(/* id 62, wireType 2 =*/498).fork(), q + 1).ldelim();
                    if (message.paneAlpha != null && Object.hasOwnProperty.call(message, "paneAlpha"))
                        $root.eyepiece.rpc.v1.PaneAlpha.encode(message.paneAlpha, writer.uint32(/* id 63, wireType 2 =*/506).fork(), q + 1).ldelim();
                    if (message.paneAlphaAck != null && Object.hasOwnProperty.call(message, "paneAlphaAck"))
                        $root.eyepiece.rpc.v1.Acknowledgement.encode(message.paneAlphaAck, writer.uint32(/* id 64, wireType 2 =*/514).fork(), q + 1).ldelim();
                    if (message.configure != null && Object.hasOwnProperty.call(message, "configure"))
                        $root.eyepiece.rpc.v1.Configure.encode(message.configure, writer.uint32(/* id 65, wireType 2 =*/522).fork(), q + 1).ldelim();
                    if (message.configureAck != null && Object.hasOwnProperty.call(message, "configureAck"))
                        $root.eyepiece.rpc.v1.Acknowledgement.encode(message.configureAck, writer.uint32(/* id 66, wireType 2 =*/530).fork(), q + 1).ldelim();
                    return writer;
                };

                /**
                 * Encodes the specified Envelope message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Envelope.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @static
                 * @param {eyepiece.rpc.v1.IEnvelope} message Envelope message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Envelope.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes an Envelope message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.Envelope} Envelope
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Envelope.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.Envelope();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                message.id = reader.string();
                                break;
                            }
                        case 2: {
                                message.sender = reader.string();
                                break;
                            }
                        case 3: {
                                message.recipient = reader.string();
                                break;
                            }
                        case 10: {
                                message.getState = $root.eyepiece.rpc.v1.GetState.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 11: {
                                message.setProperties = $root.eyepiece.rpc.v1.SetProperties.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 12: {
                                message.start = $root.eyepiece.rpc.v1.Start.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 13: {
                                message.stop = $root.eyepiece.rpc.v1.Stop.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 14: {
                                message.restart = $root.eyepiece.rpc.v1.Restart.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 15: {
                                message.recenter = $root.eyepiece.rpc.v1.Recenter.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 16: {
                                message.forceIdr = $root.eyepiece.rpc.v1.ForceIdr.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 17: {
                                message.setPreview = $root.eyepiece.rpc.v1.SetPreview.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 18: {
                                message.setCommandLine = $root.eyepiece.rpc.v1.SetCommandLine.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 19: {
                                message.listKmsOutputs = $root.eyepiece.rpc.v1.ListKmsOutputs.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 30: {
                                message.state = $root.eyepiece.rpc.v1.State.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 31: {
                                message.error = $root.eyepiece.rpc.v1.Error.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 32: {
                                message.log = $root.eyepiece.rpc.v1.Log.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 33: {
                                message.pose = $root.eyepiece.rpc.v1.Pose.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 34: {
                                message.metrics = $root.eyepiece.rpc.v1.Metrics.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 35: {
                                message.preview = $root.eyepiece.rpc.v1.Preview.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 36: {
                                message.acknowledged = reader.bool();
                                break;
                            }
                        case 37: {
                                message.kmsOutputs = $root.eyepiece.rpc.v1.KmsOutputs.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 50: {
                                message.video = $root.eyepiece.rpc.v1.Video.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 51: {
                                message.ping = $root.eyepiece.rpc.v1.Ping.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 52: {
                                message.control = $root.eyepiece.rpc.v1.Control.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 56: {
                                message.guiAlpha = $root.eyepiece.rpc.v1.GuiAlpha.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 57: {
                                message.pong = $root.eyepiece.rpc.v1.Pong.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 58: {
                                message.frame = $root.eyepiece.rpc.v1.Frame.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 59: {
                                message.info = $root.eyepiece.rpc.v1.Info.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 60: {
                                message.keyRequest = $root.eyepiece.rpc.v1.KeyRequest.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 62: {
                                message.guiAlphaAck = $root.eyepiece.rpc.v1.Acknowledgement.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 63: {
                                message.paneAlpha = $root.eyepiece.rpc.v1.PaneAlpha.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 64: {
                                message.paneAlphaAck = $root.eyepiece.rpc.v1.Acknowledgement.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 65: {
                                message.configure = $root.eyepiece.rpc.v1.Configure.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 66: {
                                message.configureAck = $root.eyepiece.rpc.v1.Acknowledgement.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes an Envelope message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.Envelope} Envelope
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Envelope.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies an Envelope message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                Envelope.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    let properties = {};
                    if (message.id != null && Object.hasOwnProperty.call(message, "id")) {
                        properties._id = 1;
                        if (!$util.isString(message.id))
                            return "id: string expected";
                    }
                    if (message.sender != null && Object.hasOwnProperty.call(message, "sender")) {
                        properties._sender = 1;
                        if (!$util.isString(message.sender))
                            return "sender: string expected";
                    }
                    if (message.recipient != null && Object.hasOwnProperty.call(message, "recipient")) {
                        properties._recipient = 1;
                        if (!$util.isString(message.recipient))
                            return "recipient: string expected";
                    }
                    if (message.getState != null && Object.hasOwnProperty.call(message, "getState")) {
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.GetState.verify(message.getState, long + 1);
                            if (error)
                                return "getState." + error;
                        }
                    }
                    if (message.setProperties != null && Object.hasOwnProperty.call(message, "setProperties")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.SetProperties.verify(message.setProperties, long + 1);
                            if (error)
                                return "setProperties." + error;
                        }
                    }
                    if (message.start != null && Object.hasOwnProperty.call(message, "start")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.Start.verify(message.start, long + 1);
                            if (error)
                                return "start." + error;
                        }
                    }
                    if (message.stop != null && Object.hasOwnProperty.call(message, "stop")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.Stop.verify(message.stop, long + 1);
                            if (error)
                                return "stop." + error;
                        }
                    }
                    if (message.restart != null && Object.hasOwnProperty.call(message, "restart")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.Restart.verify(message.restart, long + 1);
                            if (error)
                                return "restart." + error;
                        }
                    }
                    if (message.recenter != null && Object.hasOwnProperty.call(message, "recenter")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.Recenter.verify(message.recenter, long + 1);
                            if (error)
                                return "recenter." + error;
                        }
                    }
                    if (message.forceIdr != null && Object.hasOwnProperty.call(message, "forceIdr")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.ForceIdr.verify(message.forceIdr, long + 1);
                            if (error)
                                return "forceIdr." + error;
                        }
                    }
                    if (message.setPreview != null && Object.hasOwnProperty.call(message, "setPreview")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.SetPreview.verify(message.setPreview, long + 1);
                            if (error)
                                return "setPreview." + error;
                        }
                    }
                    if (message.setCommandLine != null && Object.hasOwnProperty.call(message, "setCommandLine")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.SetCommandLine.verify(message.setCommandLine, long + 1);
                            if (error)
                                return "setCommandLine." + error;
                        }
                    }
                    if (message.listKmsOutputs != null && Object.hasOwnProperty.call(message, "listKmsOutputs")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.ListKmsOutputs.verify(message.listKmsOutputs, long + 1);
                            if (error)
                                return "listKmsOutputs." + error;
                        }
                    }
                    if (message.state != null && Object.hasOwnProperty.call(message, "state")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.State.verify(message.state, long + 1);
                            if (error)
                                return "state." + error;
                        }
                    }
                    if (message.error != null && Object.hasOwnProperty.call(message, "error")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.Error.verify(message.error, long + 1);
                            if (error)
                                return "error." + error;
                        }
                    }
                    if (message.log != null && Object.hasOwnProperty.call(message, "log")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.Log.verify(message.log, long + 1);
                            if (error)
                                return "log." + error;
                        }
                    }
                    if (message.pose != null && Object.hasOwnProperty.call(message, "pose")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.Pose.verify(message.pose, long + 1);
                            if (error)
                                return "pose." + error;
                        }
                    }
                    if (message.metrics != null && Object.hasOwnProperty.call(message, "metrics")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.Metrics.verify(message.metrics, long + 1);
                            if (error)
                                return "metrics." + error;
                        }
                    }
                    if (message.preview != null && Object.hasOwnProperty.call(message, "preview")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.Preview.verify(message.preview, long + 1);
                            if (error)
                                return "preview." + error;
                        }
                    }
                    if (message.acknowledged != null && Object.hasOwnProperty.call(message, "acknowledged")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        if (typeof message.acknowledged !== "boolean")
                            return "acknowledged: boolean expected";
                    }
                    if (message.kmsOutputs != null && Object.hasOwnProperty.call(message, "kmsOutputs")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.KmsOutputs.verify(message.kmsOutputs, long + 1);
                            if (error)
                                return "kmsOutputs." + error;
                        }
                    }
                    if (message.video != null && Object.hasOwnProperty.call(message, "video")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.Video.verify(message.video, long + 1);
                            if (error)
                                return "video." + error;
                        }
                    }
                    if (message.ping != null && Object.hasOwnProperty.call(message, "ping")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.Ping.verify(message.ping, long + 1);
                            if (error)
                                return "ping." + error;
                        }
                    }
                    if (message.control != null && Object.hasOwnProperty.call(message, "control")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.Control.verify(message.control, long + 1);
                            if (error)
                                return "control." + error;
                        }
                    }
                    if (message.guiAlpha != null && Object.hasOwnProperty.call(message, "guiAlpha")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.GuiAlpha.verify(message.guiAlpha, long + 1);
                            if (error)
                                return "guiAlpha." + error;
                        }
                    }
                    if (message.pong != null && Object.hasOwnProperty.call(message, "pong")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.Pong.verify(message.pong, long + 1);
                            if (error)
                                return "pong." + error;
                        }
                    }
                    if (message.frame != null && Object.hasOwnProperty.call(message, "frame")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.Frame.verify(message.frame, long + 1);
                            if (error)
                                return "frame." + error;
                        }
                    }
                    if (message.info != null && Object.hasOwnProperty.call(message, "info")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.Info.verify(message.info, long + 1);
                            if (error)
                                return "info." + error;
                        }
                    }
                    if (message.keyRequest != null && Object.hasOwnProperty.call(message, "keyRequest")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.KeyRequest.verify(message.keyRequest, long + 1);
                            if (error)
                                return "keyRequest." + error;
                        }
                    }
                    if (message.guiAlphaAck != null && Object.hasOwnProperty.call(message, "guiAlphaAck")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.Acknowledgement.verify(message.guiAlphaAck, long + 1);
                            if (error)
                                return "guiAlphaAck." + error;
                        }
                    }
                    if (message.paneAlpha != null && Object.hasOwnProperty.call(message, "paneAlpha")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.PaneAlpha.verify(message.paneAlpha, long + 1);
                            if (error)
                                return "paneAlpha." + error;
                        }
                    }
                    if (message.paneAlphaAck != null && Object.hasOwnProperty.call(message, "paneAlphaAck")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.Acknowledgement.verify(message.paneAlphaAck, long + 1);
                            if (error)
                                return "paneAlphaAck." + error;
                        }
                    }
                    if (message.configure != null && Object.hasOwnProperty.call(message, "configure")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.Configure.verify(message.configure, long + 1);
                            if (error)
                                return "configure." + error;
                        }
                    }
                    if (message.configureAck != null && Object.hasOwnProperty.call(message, "configureAck")) {
                        if (properties.message === 1)
                            return "message: multiple values";
                        properties.message = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.Acknowledgement.verify(message.configureAck, long + 1);
                            if (error)
                                return "configureAck." + error;
                        }
                    }
                    return null;
                };

                /**
                 * Creates an Envelope message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.Envelope} Envelope
                 */
                Envelope.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.Envelope)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.Envelope: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.Envelope();
                    if (object.id != null)
                        message.id = String(object.id);
                    if (object.sender != null)
                        message.sender = String(object.sender);
                    if (object.recipient != null)
                        message.recipient = String(object.recipient);
                    if (object.getState != null) {
                        if (!$util.isObject(object.getState))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.getState: object expected");
                        message.getState = $root.eyepiece.rpc.v1.GetState.fromObject(object.getState, long + 1);
                    }
                    if (object.setProperties != null) {
                        if (!$util.isObject(object.setProperties))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.setProperties: object expected");
                        message.setProperties = $root.eyepiece.rpc.v1.SetProperties.fromObject(object.setProperties, long + 1);
                    }
                    if (object.start != null) {
                        if (!$util.isObject(object.start))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.start: object expected");
                        message.start = $root.eyepiece.rpc.v1.Start.fromObject(object.start, long + 1);
                    }
                    if (object.stop != null) {
                        if (!$util.isObject(object.stop))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.stop: object expected");
                        message.stop = $root.eyepiece.rpc.v1.Stop.fromObject(object.stop, long + 1);
                    }
                    if (object.restart != null) {
                        if (!$util.isObject(object.restart))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.restart: object expected");
                        message.restart = $root.eyepiece.rpc.v1.Restart.fromObject(object.restart, long + 1);
                    }
                    if (object.recenter != null) {
                        if (!$util.isObject(object.recenter))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.recenter: object expected");
                        message.recenter = $root.eyepiece.rpc.v1.Recenter.fromObject(object.recenter, long + 1);
                    }
                    if (object.forceIdr != null) {
                        if (!$util.isObject(object.forceIdr))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.forceIdr: object expected");
                        message.forceIdr = $root.eyepiece.rpc.v1.ForceIdr.fromObject(object.forceIdr, long + 1);
                    }
                    if (object.setPreview != null) {
                        if (!$util.isObject(object.setPreview))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.setPreview: object expected");
                        message.setPreview = $root.eyepiece.rpc.v1.SetPreview.fromObject(object.setPreview, long + 1);
                    }
                    if (object.setCommandLine != null) {
                        if (!$util.isObject(object.setCommandLine))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.setCommandLine: object expected");
                        message.setCommandLine = $root.eyepiece.rpc.v1.SetCommandLine.fromObject(object.setCommandLine, long + 1);
                    }
                    if (object.listKmsOutputs != null) {
                        if (!$util.isObject(object.listKmsOutputs))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.listKmsOutputs: object expected");
                        message.listKmsOutputs = $root.eyepiece.rpc.v1.ListKmsOutputs.fromObject(object.listKmsOutputs, long + 1);
                    }
                    if (object.state != null) {
                        if (!$util.isObject(object.state))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.state: object expected");
                        message.state = $root.eyepiece.rpc.v1.State.fromObject(object.state, long + 1);
                    }
                    if (object.error != null) {
                        if (!$util.isObject(object.error))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.error: object expected");
                        message.error = $root.eyepiece.rpc.v1.Error.fromObject(object.error, long + 1);
                    }
                    if (object.log != null) {
                        if (!$util.isObject(object.log))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.log: object expected");
                        message.log = $root.eyepiece.rpc.v1.Log.fromObject(object.log, long + 1);
                    }
                    if (object.pose != null) {
                        if (!$util.isObject(object.pose))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.pose: object expected");
                        message.pose = $root.eyepiece.rpc.v1.Pose.fromObject(object.pose, long + 1);
                    }
                    if (object.metrics != null) {
                        if (!$util.isObject(object.metrics))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.metrics: object expected");
                        message.metrics = $root.eyepiece.rpc.v1.Metrics.fromObject(object.metrics, long + 1);
                    }
                    if (object.preview != null) {
                        if (!$util.isObject(object.preview))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.preview: object expected");
                        message.preview = $root.eyepiece.rpc.v1.Preview.fromObject(object.preview, long + 1);
                    }
                    if (object.acknowledged != null)
                        message.acknowledged = Boolean(object.acknowledged);
                    if (object.kmsOutputs != null) {
                        if (!$util.isObject(object.kmsOutputs))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.kmsOutputs: object expected");
                        message.kmsOutputs = $root.eyepiece.rpc.v1.KmsOutputs.fromObject(object.kmsOutputs, long + 1);
                    }
                    if (object.video != null) {
                        if (!$util.isObject(object.video))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.video: object expected");
                        message.video = $root.eyepiece.rpc.v1.Video.fromObject(object.video, long + 1);
                    }
                    if (object.ping != null) {
                        if (!$util.isObject(object.ping))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.ping: object expected");
                        message.ping = $root.eyepiece.rpc.v1.Ping.fromObject(object.ping, long + 1);
                    }
                    if (object.control != null) {
                        if (!$util.isObject(object.control))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.control: object expected");
                        message.control = $root.eyepiece.rpc.v1.Control.fromObject(object.control, long + 1);
                    }
                    if (object.guiAlpha != null) {
                        if (!$util.isObject(object.guiAlpha))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.guiAlpha: object expected");
                        message.guiAlpha = $root.eyepiece.rpc.v1.GuiAlpha.fromObject(object.guiAlpha, long + 1);
                    }
                    if (object.pong != null) {
                        if (!$util.isObject(object.pong))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.pong: object expected");
                        message.pong = $root.eyepiece.rpc.v1.Pong.fromObject(object.pong, long + 1);
                    }
                    if (object.frame != null) {
                        if (!$util.isObject(object.frame))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.frame: object expected");
                        message.frame = $root.eyepiece.rpc.v1.Frame.fromObject(object.frame, long + 1);
                    }
                    if (object.info != null) {
                        if (!$util.isObject(object.info))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.info: object expected");
                        message.info = $root.eyepiece.rpc.v1.Info.fromObject(object.info, long + 1);
                    }
                    if (object.keyRequest != null) {
                        if (!$util.isObject(object.keyRequest))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.keyRequest: object expected");
                        message.keyRequest = $root.eyepiece.rpc.v1.KeyRequest.fromObject(object.keyRequest, long + 1);
                    }
                    if (object.guiAlphaAck != null) {
                        if (!$util.isObject(object.guiAlphaAck))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.guiAlphaAck: object expected");
                        message.guiAlphaAck = $root.eyepiece.rpc.v1.Acknowledgement.fromObject(object.guiAlphaAck, long + 1);
                    }
                    if (object.paneAlpha != null) {
                        if (!$util.isObject(object.paneAlpha))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.paneAlpha: object expected");
                        message.paneAlpha = $root.eyepiece.rpc.v1.PaneAlpha.fromObject(object.paneAlpha, long + 1);
                    }
                    if (object.paneAlphaAck != null) {
                        if (!$util.isObject(object.paneAlphaAck))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.paneAlphaAck: object expected");
                        message.paneAlphaAck = $root.eyepiece.rpc.v1.Acknowledgement.fromObject(object.paneAlphaAck, long + 1);
                    }
                    if (object.configure != null) {
                        if (!$util.isObject(object.configure))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.configure: object expected");
                        message.configure = $root.eyepiece.rpc.v1.Configure.fromObject(object.configure, long + 1);
                    }
                    if (object.configureAck != null) {
                        if (!$util.isObject(object.configureAck))
                            throw TypeError(".eyepiece.rpc.v1.Envelope.configureAck: object expected");
                        message.configureAck = $root.eyepiece.rpc.v1.Acknowledgement.fromObject(object.configureAck, long + 1);
                    }
                    return message;
                };

                /**
                 * Creates a plain object from an Envelope message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @static
                 * @param {eyepiece.rpc.v1.Envelope} message Envelope
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                Envelope.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (message.id != null && Object.hasOwnProperty.call(message, "id")) {
                        object.id = message.id;
                        if (options.oneofs)
                            object._id = "id";
                    }
                    if (message.sender != null && Object.hasOwnProperty.call(message, "sender")) {
                        object.sender = message.sender;
                        if (options.oneofs)
                            object._sender = "sender";
                    }
                    if (message.recipient != null && Object.hasOwnProperty.call(message, "recipient")) {
                        object.recipient = message.recipient;
                        if (options.oneofs)
                            object._recipient = "recipient";
                    }
                    if (message.getState != null && Object.hasOwnProperty.call(message, "getState")) {
                        object.getState = $root.eyepiece.rpc.v1.GetState.toObject(message.getState, options, q + 1);
                        if (options.oneofs)
                            object.message = "getState";
                    }
                    if (message.setProperties != null && Object.hasOwnProperty.call(message, "setProperties")) {
                        object.setProperties = $root.eyepiece.rpc.v1.SetProperties.toObject(message.setProperties, options, q + 1);
                        if (options.oneofs)
                            object.message = "setProperties";
                    }
                    if (message.start != null && Object.hasOwnProperty.call(message, "start")) {
                        object.start = $root.eyepiece.rpc.v1.Start.toObject(message.start, options, q + 1);
                        if (options.oneofs)
                            object.message = "start";
                    }
                    if (message.stop != null && Object.hasOwnProperty.call(message, "stop")) {
                        object.stop = $root.eyepiece.rpc.v1.Stop.toObject(message.stop, options, q + 1);
                        if (options.oneofs)
                            object.message = "stop";
                    }
                    if (message.restart != null && Object.hasOwnProperty.call(message, "restart")) {
                        object.restart = $root.eyepiece.rpc.v1.Restart.toObject(message.restart, options, q + 1);
                        if (options.oneofs)
                            object.message = "restart";
                    }
                    if (message.recenter != null && Object.hasOwnProperty.call(message, "recenter")) {
                        object.recenter = $root.eyepiece.rpc.v1.Recenter.toObject(message.recenter, options, q + 1);
                        if (options.oneofs)
                            object.message = "recenter";
                    }
                    if (message.forceIdr != null && Object.hasOwnProperty.call(message, "forceIdr")) {
                        object.forceIdr = $root.eyepiece.rpc.v1.ForceIdr.toObject(message.forceIdr, options, q + 1);
                        if (options.oneofs)
                            object.message = "forceIdr";
                    }
                    if (message.setPreview != null && Object.hasOwnProperty.call(message, "setPreview")) {
                        object.setPreview = $root.eyepiece.rpc.v1.SetPreview.toObject(message.setPreview, options, q + 1);
                        if (options.oneofs)
                            object.message = "setPreview";
                    }
                    if (message.setCommandLine != null && Object.hasOwnProperty.call(message, "setCommandLine")) {
                        object.setCommandLine = $root.eyepiece.rpc.v1.SetCommandLine.toObject(message.setCommandLine, options, q + 1);
                        if (options.oneofs)
                            object.message = "setCommandLine";
                    }
                    if (message.listKmsOutputs != null && Object.hasOwnProperty.call(message, "listKmsOutputs")) {
                        object.listKmsOutputs = $root.eyepiece.rpc.v1.ListKmsOutputs.toObject(message.listKmsOutputs, options, q + 1);
                        if (options.oneofs)
                            object.message = "listKmsOutputs";
                    }
                    if (message.state != null && Object.hasOwnProperty.call(message, "state")) {
                        object.state = $root.eyepiece.rpc.v1.State.toObject(message.state, options, q + 1);
                        if (options.oneofs)
                            object.message = "state";
                    }
                    if (message.error != null && Object.hasOwnProperty.call(message, "error")) {
                        object.error = $root.eyepiece.rpc.v1.Error.toObject(message.error, options, q + 1);
                        if (options.oneofs)
                            object.message = "error";
                    }
                    if (message.log != null && Object.hasOwnProperty.call(message, "log")) {
                        object.log = $root.eyepiece.rpc.v1.Log.toObject(message.log, options, q + 1);
                        if (options.oneofs)
                            object.message = "log";
                    }
                    if (message.pose != null && Object.hasOwnProperty.call(message, "pose")) {
                        object.pose = $root.eyepiece.rpc.v1.Pose.toObject(message.pose, options, q + 1);
                        if (options.oneofs)
                            object.message = "pose";
                    }
                    if (message.metrics != null && Object.hasOwnProperty.call(message, "metrics")) {
                        object.metrics = $root.eyepiece.rpc.v1.Metrics.toObject(message.metrics, options, q + 1);
                        if (options.oneofs)
                            object.message = "metrics";
                    }
                    if (message.preview != null && Object.hasOwnProperty.call(message, "preview")) {
                        object.preview = $root.eyepiece.rpc.v1.Preview.toObject(message.preview, options, q + 1);
                        if (options.oneofs)
                            object.message = "preview";
                    }
                    if (message.acknowledged != null && Object.hasOwnProperty.call(message, "acknowledged")) {
                        object.acknowledged = message.acknowledged;
                        if (options.oneofs)
                            object.message = "acknowledged";
                    }
                    if (message.kmsOutputs != null && Object.hasOwnProperty.call(message, "kmsOutputs")) {
                        object.kmsOutputs = $root.eyepiece.rpc.v1.KmsOutputs.toObject(message.kmsOutputs, options, q + 1);
                        if (options.oneofs)
                            object.message = "kmsOutputs";
                    }
                    if (message.video != null && Object.hasOwnProperty.call(message, "video")) {
                        object.video = $root.eyepiece.rpc.v1.Video.toObject(message.video, options, q + 1);
                        if (options.oneofs)
                            object.message = "video";
                    }
                    if (message.ping != null && Object.hasOwnProperty.call(message, "ping")) {
                        object.ping = $root.eyepiece.rpc.v1.Ping.toObject(message.ping, options, q + 1);
                        if (options.oneofs)
                            object.message = "ping";
                    }
                    if (message.control != null && Object.hasOwnProperty.call(message, "control")) {
                        object.control = $root.eyepiece.rpc.v1.Control.toObject(message.control, options, q + 1);
                        if (options.oneofs)
                            object.message = "control";
                    }
                    if (message.guiAlpha != null && Object.hasOwnProperty.call(message, "guiAlpha")) {
                        object.guiAlpha = $root.eyepiece.rpc.v1.GuiAlpha.toObject(message.guiAlpha, options, q + 1);
                        if (options.oneofs)
                            object.message = "guiAlpha";
                    }
                    if (message.pong != null && Object.hasOwnProperty.call(message, "pong")) {
                        object.pong = $root.eyepiece.rpc.v1.Pong.toObject(message.pong, options, q + 1);
                        if (options.oneofs)
                            object.message = "pong";
                    }
                    if (message.frame != null && Object.hasOwnProperty.call(message, "frame")) {
                        object.frame = $root.eyepiece.rpc.v1.Frame.toObject(message.frame, options, q + 1);
                        if (options.oneofs)
                            object.message = "frame";
                    }
                    if (message.info != null && Object.hasOwnProperty.call(message, "info")) {
                        object.info = $root.eyepiece.rpc.v1.Info.toObject(message.info, options, q + 1);
                        if (options.oneofs)
                            object.message = "info";
                    }
                    if (message.keyRequest != null && Object.hasOwnProperty.call(message, "keyRequest")) {
                        object.keyRequest = $root.eyepiece.rpc.v1.KeyRequest.toObject(message.keyRequest, options, q + 1);
                        if (options.oneofs)
                            object.message = "keyRequest";
                    }
                    if (message.guiAlphaAck != null && Object.hasOwnProperty.call(message, "guiAlphaAck")) {
                        object.guiAlphaAck = $root.eyepiece.rpc.v1.Acknowledgement.toObject(message.guiAlphaAck, options, q + 1);
                        if (options.oneofs)
                            object.message = "guiAlphaAck";
                    }
                    if (message.paneAlpha != null && Object.hasOwnProperty.call(message, "paneAlpha")) {
                        object.paneAlpha = $root.eyepiece.rpc.v1.PaneAlpha.toObject(message.paneAlpha, options, q + 1);
                        if (options.oneofs)
                            object.message = "paneAlpha";
                    }
                    if (message.paneAlphaAck != null && Object.hasOwnProperty.call(message, "paneAlphaAck")) {
                        object.paneAlphaAck = $root.eyepiece.rpc.v1.Acknowledgement.toObject(message.paneAlphaAck, options, q + 1);
                        if (options.oneofs)
                            object.message = "paneAlphaAck";
                    }
                    if (message.configure != null && Object.hasOwnProperty.call(message, "configure")) {
                        object.configure = $root.eyepiece.rpc.v1.Configure.toObject(message.configure, options, q + 1);
                        if (options.oneofs)
                            object.message = "configure";
                    }
                    if (message.configureAck != null && Object.hasOwnProperty.call(message, "configureAck")) {
                        object.configureAck = $root.eyepiece.rpc.v1.Acknowledgement.toObject(message.configureAck, options, q + 1);
                        if (options.oneofs)
                            object.message = "configureAck";
                    }
                    return object;
                };

                /**
                 * Converts this Envelope to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                Envelope.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for Envelope
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.Envelope
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                Envelope.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.Envelope";
                };

                return Envelope;
            })();

            v1.StringList = (function() {

                /**
                 * Properties of a StringList.
                 * @memberof eyepiece.rpc.v1
                 * @interface IStringList
                 * @property {Array.<string>|null} [values] StringList values
                 */

                /**
                 * Constructs a new StringList.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a StringList.
                 * @implements IStringList
                 * @constructor
                 * @param {eyepiece.rpc.v1.IStringList=} [properties] Properties to set
                 */
                function StringList(properties) {
                    this.values = [];
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * StringList values.
                 * @member {Array.<string>} values
                 * @memberof eyepiece.rpc.v1.StringList
                 * @instance
                 */
                StringList.prototype.values = $util.emptyArray;

                /**
                 * Creates a new StringList instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.StringList
                 * @static
                 * @param {eyepiece.rpc.v1.IStringList=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.StringList} StringList instance
                 */
                StringList.create = function create(properties) {
                    return new StringList(properties);
                };

                /**
                 * Encodes the specified StringList message. Does not implicitly {@link eyepiece.rpc.v1.StringList.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.StringList
                 * @static
                 * @param {eyepiece.rpc.v1.IStringList} message StringList message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                StringList.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.values != null && message.values.length)
                        for (let i = 0; i < message.values.length; ++i)
                            writer.uint32(/* id 1, wireType 2 =*/10).string(message.values[i]);
                    return writer;
                };

                /**
                 * Encodes the specified StringList message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.StringList.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.StringList
                 * @static
                 * @param {eyepiece.rpc.v1.IStringList} message StringList message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                StringList.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a StringList message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.StringList
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.StringList} StringList
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                StringList.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.StringList();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                if (!(message.values && message.values.length))
                                    message.values = [];
                                message.values.push(reader.string());
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a StringList message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.StringList
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.StringList} StringList
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                StringList.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a StringList message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.StringList
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                StringList.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.values != null && Object.hasOwnProperty.call(message, "values")) {
                        if (!Array.isArray(message.values))
                            return "values: array expected";
                        for (let i = 0; i < message.values.length; ++i)
                            if (!$util.isString(message.values[i]))
                                return "values: string[] expected";
                    }
                    return null;
                };

                /**
                 * Creates a StringList message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.StringList
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.StringList} StringList
                 */
                StringList.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.StringList)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.StringList: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.StringList();
                    if (object.values) {
                        if (!Array.isArray(object.values))
                            throw TypeError(".eyepiece.rpc.v1.StringList.values: array expected");
                        message.values = [];
                        for (let i = 0; i < object.values.length; ++i)
                            message.values[i] = String(object.values[i]);
                    }
                    return message;
                };

                /**
                 * Creates a plain object from a StringList message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.StringList
                 * @static
                 * @param {eyepiece.rpc.v1.StringList} message StringList
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                StringList.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.arrays || options.defaults)
                        object.values = [];
                    if (message.values && message.values.length) {
                        object.values = [];
                        for (let j = 0; j < message.values.length; ++j)
                            object.values[j] = message.values[j];
                    }
                    return object;
                };

                /**
                 * Converts this StringList to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.StringList
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                StringList.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for StringList
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.StringList
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                StringList.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.StringList";
                };

                return StringList;
            })();

            v1.Value = (function() {

                /**
                 * Properties of a Value.
                 * @memberof eyepiece.rpc.v1
                 * @interface IValue
                 * @property {boolean|null} [boolean] Value boolean
                 * @property {number|Long|null} [integer] Value integer
                 * @property {number|null} [number] Value number
                 * @property {string|null} [text] Value text
                 * @property {eyepiece.rpc.v1.IStringList|null} [strings] Value strings
                 * @property {boolean|null} [unset] Value unset
                 */

                /**
                 * Constructs a new Value.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a Value.
                 * @implements IValue
                 * @constructor
                 * @param {eyepiece.rpc.v1.IValue=} [properties] Properties to set
                 */
                function Value(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Value boolean.
                 * @member {boolean|null|undefined} boolean
                 * @memberof eyepiece.rpc.v1.Value
                 * @instance
                 */
                Value.prototype.boolean = null;

                /**
                 * Value integer.
                 * @member {number|Long|null|undefined} integer
                 * @memberof eyepiece.rpc.v1.Value
                 * @instance
                 */
                Value.prototype.integer = null;

                /**
                 * Value number.
                 * @member {number|null|undefined} number
                 * @memberof eyepiece.rpc.v1.Value
                 * @instance
                 */
                Value.prototype.number = null;

                /**
                 * Value text.
                 * @member {string|null|undefined} text
                 * @memberof eyepiece.rpc.v1.Value
                 * @instance
                 */
                Value.prototype.text = null;

                /**
                 * Value strings.
                 * @member {eyepiece.rpc.v1.IStringList|null|undefined} strings
                 * @memberof eyepiece.rpc.v1.Value
                 * @instance
                 */
                Value.prototype.strings = null;

                /**
                 * Value unset.
                 * @member {boolean|null|undefined} unset
                 * @memberof eyepiece.rpc.v1.Value
                 * @instance
                 */
                Value.prototype.unset = null;

                // OneOf field names bound to virtual getters and setters
                let $oneOfFields;

                /**
                 * Value kind.
                 * @member {"boolean"|"integer"|"number"|"text"|"strings"|"unset"|undefined} kind
                 * @memberof eyepiece.rpc.v1.Value
                 * @instance
                 */
                Object.defineProperty(Value.prototype, "kind", {
                    get: $util.oneOfGetter($oneOfFields = ["boolean", "integer", "number", "text", "strings", "unset"]),
                    set: $util.oneOfSetter($oneOfFields)
                });

                /**
                 * Creates a new Value instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.Value
                 * @static
                 * @param {eyepiece.rpc.v1.IValue=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.Value} Value instance
                 */
                Value.create = function create(properties) {
                    return new Value(properties);
                };

                /**
                 * Encodes the specified Value message. Does not implicitly {@link eyepiece.rpc.v1.Value.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.Value
                 * @static
                 * @param {eyepiece.rpc.v1.IValue} message Value message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Value.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.boolean != null && Object.hasOwnProperty.call(message, "boolean"))
                        writer.uint32(/* id 1, wireType 0 =*/8).bool(message.boolean);
                    if (message.integer != null && Object.hasOwnProperty.call(message, "integer"))
                        writer.uint32(/* id 2, wireType 0 =*/16).sint64(message.integer);
                    if (message.number != null && Object.hasOwnProperty.call(message, "number"))
                        writer.uint32(/* id 3, wireType 1 =*/25).double(message.number);
                    if (message.text != null && Object.hasOwnProperty.call(message, "text"))
                        writer.uint32(/* id 4, wireType 2 =*/34).string(message.text);
                    if (message.strings != null && Object.hasOwnProperty.call(message, "strings"))
                        $root.eyepiece.rpc.v1.StringList.encode(message.strings, writer.uint32(/* id 5, wireType 2 =*/42).fork(), q + 1).ldelim();
                    if (message.unset != null && Object.hasOwnProperty.call(message, "unset"))
                        writer.uint32(/* id 6, wireType 0 =*/48).bool(message.unset);
                    return writer;
                };

                /**
                 * Encodes the specified Value message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Value.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.Value
                 * @static
                 * @param {eyepiece.rpc.v1.IValue} message Value message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Value.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a Value message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.Value
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.Value} Value
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Value.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.Value();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                message.boolean = reader.bool();
                                break;
                            }
                        case 2: {
                                message.integer = reader.sint64();
                                break;
                            }
                        case 3: {
                                message.number = reader.double();
                                break;
                            }
                        case 4: {
                                message.text = reader.string();
                                break;
                            }
                        case 5: {
                                message.strings = $root.eyepiece.rpc.v1.StringList.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        case 6: {
                                message.unset = reader.bool();
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a Value message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.Value
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.Value} Value
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Value.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a Value message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.Value
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                Value.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    let properties = {};
                    if (message.boolean != null && Object.hasOwnProperty.call(message, "boolean")) {
                        properties.kind = 1;
                        if (typeof message.boolean !== "boolean")
                            return "boolean: boolean expected";
                    }
                    if (message.integer != null && Object.hasOwnProperty.call(message, "integer")) {
                        if (properties.kind === 1)
                            return "kind: multiple values";
                        properties.kind = 1;
                        if (!$util.isInteger(message.integer) && !(message.integer && $util.isInteger(message.integer.low) && $util.isInteger(message.integer.high)))
                            return "integer: integer|Long expected";
                    }
                    if (message.number != null && Object.hasOwnProperty.call(message, "number")) {
                        if (properties.kind === 1)
                            return "kind: multiple values";
                        properties.kind = 1;
                        if (typeof message.number !== "number")
                            return "number: number expected";
                    }
                    if (message.text != null && Object.hasOwnProperty.call(message, "text")) {
                        if (properties.kind === 1)
                            return "kind: multiple values";
                        properties.kind = 1;
                        if (!$util.isString(message.text))
                            return "text: string expected";
                    }
                    if (message.strings != null && Object.hasOwnProperty.call(message, "strings")) {
                        if (properties.kind === 1)
                            return "kind: multiple values";
                        properties.kind = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.StringList.verify(message.strings, long + 1);
                            if (error)
                                return "strings." + error;
                        }
                    }
                    if (message.unset != null && Object.hasOwnProperty.call(message, "unset")) {
                        if (properties.kind === 1)
                            return "kind: multiple values";
                        properties.kind = 1;
                        if (typeof message.unset !== "boolean")
                            return "unset: boolean expected";
                    }
                    return null;
                };

                /**
                 * Creates a Value message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.Value
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.Value} Value
                 */
                Value.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.Value)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.Value: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.Value();
                    if (object.boolean != null)
                        message.boolean = Boolean(object.boolean);
                    if (object.integer != null)
                        if ($util.Long)
                            message.integer = $util.Long.fromValue(object.integer, false);
                        else if (typeof object.integer === "string")
                            message.integer = parseInt(object.integer, 10);
                        else if (typeof object.integer === "number")
                            message.integer = object.integer;
                        else if (typeof object.integer === "object")
                            message.integer = new $util.LongBits(object.integer.low >>> 0, object.integer.high >>> 0).toNumber();
                    if (object.number != null)
                        message.number = Number(object.number);
                    if (object.text != null)
                        message.text = String(object.text);
                    if (object.strings != null) {
                        if (!$util.isObject(object.strings))
                            throw TypeError(".eyepiece.rpc.v1.Value.strings: object expected");
                        message.strings = $root.eyepiece.rpc.v1.StringList.fromObject(object.strings, long + 1);
                    }
                    if (object.unset != null)
                        message.unset = Boolean(object.unset);
                    return message;
                };

                /**
                 * Creates a plain object from a Value message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.Value
                 * @static
                 * @param {eyepiece.rpc.v1.Value} message Value
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                Value.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (message.boolean != null && Object.hasOwnProperty.call(message, "boolean")) {
                        object.boolean = message.boolean;
                        if (options.oneofs)
                            object.kind = "boolean";
                    }
                    if (message.integer != null && Object.hasOwnProperty.call(message, "integer")) {
                        if (typeof BigInt !== "undefined" && options.longs === BigInt)
                            object.integer = typeof message.integer === "number" ? BigInt(message.integer) : $util.Long.fromBits(message.integer.low >>> 0, message.integer.high >>> 0, false).toBigInt();
                        else if (typeof message.integer === "number")
                            object.integer = options.longs === String ? String(message.integer) : message.integer;
                        else
                            object.integer = options.longs === String ? $util.Long.prototype.toString.call(message.integer) : options.longs === Number ? new $util.LongBits(message.integer.low >>> 0, message.integer.high >>> 0).toNumber() : message.integer;
                        if (options.oneofs)
                            object.kind = "integer";
                    }
                    if (message.number != null && Object.hasOwnProperty.call(message, "number")) {
                        object.number = options.json && !isFinite(message.number) ? String(message.number) : message.number;
                        if (options.oneofs)
                            object.kind = "number";
                    }
                    if (message.text != null && Object.hasOwnProperty.call(message, "text")) {
                        object.text = message.text;
                        if (options.oneofs)
                            object.kind = "text";
                    }
                    if (message.strings != null && Object.hasOwnProperty.call(message, "strings")) {
                        object.strings = $root.eyepiece.rpc.v1.StringList.toObject(message.strings, options, q + 1);
                        if (options.oneofs)
                            object.kind = "strings";
                    }
                    if (message.unset != null && Object.hasOwnProperty.call(message, "unset")) {
                        object.unset = message.unset;
                        if (options.oneofs)
                            object.kind = "unset";
                    }
                    return object;
                };

                /**
                 * Converts this Value to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.Value
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                Value.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for Value
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.Value
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                Value.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.Value";
                };

                return Value;
            })();

            v1.Property = (function() {

                /**
                 * Properties of a Property.
                 * @memberof eyepiece.rpc.v1
                 * @interface IProperty
                 * @property {string|null} [name] Property name
                 * @property {eyepiece.rpc.v1.IValue|null} [value] Property value
                 */

                /**
                 * Constructs a new Property.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a Property.
                 * @implements IProperty
                 * @constructor
                 * @param {eyepiece.rpc.v1.IProperty=} [properties] Properties to set
                 */
                function Property(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Property name.
                 * @member {string} name
                 * @memberof eyepiece.rpc.v1.Property
                 * @instance
                 */
                Property.prototype.name = "";

                /**
                 * Property value.
                 * @member {eyepiece.rpc.v1.IValue|null|undefined} value
                 * @memberof eyepiece.rpc.v1.Property
                 * @instance
                 */
                Property.prototype.value = null;

                /**
                 * Creates a new Property instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.Property
                 * @static
                 * @param {eyepiece.rpc.v1.IProperty=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.Property} Property instance
                 */
                Property.create = function create(properties) {
                    return new Property(properties);
                };

                /**
                 * Encodes the specified Property message. Does not implicitly {@link eyepiece.rpc.v1.Property.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.Property
                 * @static
                 * @param {eyepiece.rpc.v1.IProperty} message Property message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Property.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.name != null && Object.hasOwnProperty.call(message, "name"))
                        writer.uint32(/* id 1, wireType 2 =*/10).string(message.name);
                    if (message.value != null && Object.hasOwnProperty.call(message, "value"))
                        $root.eyepiece.rpc.v1.Value.encode(message.value, writer.uint32(/* id 2, wireType 2 =*/18).fork(), q + 1).ldelim();
                    return writer;
                };

                /**
                 * Encodes the specified Property message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Property.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.Property
                 * @static
                 * @param {eyepiece.rpc.v1.IProperty} message Property message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Property.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a Property message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.Property
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.Property} Property
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Property.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.Property();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                message.name = reader.string();
                                break;
                            }
                        case 2: {
                                message.value = $root.eyepiece.rpc.v1.Value.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a Property message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.Property
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.Property} Property
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Property.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a Property message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.Property
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                Property.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.name != null && Object.hasOwnProperty.call(message, "name"))
                        if (!$util.isString(message.name))
                            return "name: string expected";
                    if (message.value != null && Object.hasOwnProperty.call(message, "value")) {
                        let error = $root.eyepiece.rpc.v1.Value.verify(message.value, long + 1);
                        if (error)
                            return "value." + error;
                    }
                    return null;
                };

                /**
                 * Creates a Property message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.Property
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.Property} Property
                 */
                Property.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.Property)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.Property: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.Property();
                    if (object.name != null)
                        message.name = String(object.name);
                    if (object.value != null) {
                        if (!$util.isObject(object.value))
                            throw TypeError(".eyepiece.rpc.v1.Property.value: object expected");
                        message.value = $root.eyepiece.rpc.v1.Value.fromObject(object.value, long + 1);
                    }
                    return message;
                };

                /**
                 * Creates a plain object from a Property message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.Property
                 * @static
                 * @param {eyepiece.rpc.v1.Property} message Property
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                Property.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.defaults) {
                        object.name = "";
                        object.value = null;
                    }
                    if (message.name != null && Object.hasOwnProperty.call(message, "name"))
                        object.name = message.name;
                    if (message.value != null && Object.hasOwnProperty.call(message, "value"))
                        object.value = $root.eyepiece.rpc.v1.Value.toObject(message.value, options, q + 1);
                    return object;
                };

                /**
                 * Converts this Property to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.Property
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                Property.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for Property
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.Property
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                Property.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.Property";
                };

                return Property;
            })();

            v1.GetState = (function() {

                /**
                 * Properties of a GetState.
                 * @memberof eyepiece.rpc.v1
                 * @interface IGetState
                 */

                /**
                 * Constructs a new GetState.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a GetState.
                 * @implements IGetState
                 * @constructor
                 * @param {eyepiece.rpc.v1.IGetState=} [properties] Properties to set
                 */
                function GetState(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Creates a new GetState instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.GetState
                 * @static
                 * @param {eyepiece.rpc.v1.IGetState=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.GetState} GetState instance
                 */
                GetState.create = function create(properties) {
                    return new GetState(properties);
                };

                /**
                 * Encodes the specified GetState message. Does not implicitly {@link eyepiece.rpc.v1.GetState.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.GetState
                 * @static
                 * @param {eyepiece.rpc.v1.IGetState} message GetState message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                GetState.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    return writer;
                };

                /**
                 * Encodes the specified GetState message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.GetState.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.GetState
                 * @static
                 * @param {eyepiece.rpc.v1.IGetState} message GetState message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                GetState.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a GetState message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.GetState
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.GetState} GetState
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                GetState.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.GetState();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a GetState message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.GetState
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.GetState} GetState
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                GetState.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a GetState message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.GetState
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                GetState.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    return null;
                };

                /**
                 * Creates a GetState message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.GetState
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.GetState} GetState
                 */
                GetState.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.GetState)
                        return object;
                    return new $root.eyepiece.rpc.v1.GetState();
                };

                /**
                 * Creates a plain object from a GetState message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.GetState
                 * @static
                 * @param {eyepiece.rpc.v1.GetState} message GetState
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                GetState.toObject = function toObject() {
                    return {};
                };

                /**
                 * Converts this GetState to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.GetState
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                GetState.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for GetState
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.GetState
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                GetState.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.GetState";
                };

                return GetState;
            })();

            v1.SetProperties = (function() {

                /**
                 * Properties of a SetProperties.
                 * @memberof eyepiece.rpc.v1
                 * @interface ISetProperties
                 * @property {Array.<eyepiece.rpc.v1.IProperty>|null} [properties] SetProperties properties
                 */

                /**
                 * Constructs a new SetProperties.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a SetProperties.
                 * @implements ISetProperties
                 * @constructor
                 * @param {eyepiece.rpc.v1.ISetProperties=} [properties] Properties to set
                 */
                function SetProperties(properties) {
                    this.properties = [];
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * SetProperties properties.
                 * @member {Array.<eyepiece.rpc.v1.IProperty>} properties
                 * @memberof eyepiece.rpc.v1.SetProperties
                 * @instance
                 */
                SetProperties.prototype.properties = $util.emptyArray;

                /**
                 * Creates a new SetProperties instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.SetProperties
                 * @static
                 * @param {eyepiece.rpc.v1.ISetProperties=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.SetProperties} SetProperties instance
                 */
                SetProperties.create = function create(properties) {
                    return new SetProperties(properties);
                };

                /**
                 * Encodes the specified SetProperties message. Does not implicitly {@link eyepiece.rpc.v1.SetProperties.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.SetProperties
                 * @static
                 * @param {eyepiece.rpc.v1.ISetProperties} message SetProperties message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                SetProperties.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.properties != null && message.properties.length)
                        for (let i = 0; i < message.properties.length; ++i)
                            $root.eyepiece.rpc.v1.Property.encode(message.properties[i], writer.uint32(/* id 1, wireType 2 =*/10).fork(), q + 1).ldelim();
                    return writer;
                };

                /**
                 * Encodes the specified SetProperties message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.SetProperties.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.SetProperties
                 * @static
                 * @param {eyepiece.rpc.v1.ISetProperties} message SetProperties message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                SetProperties.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a SetProperties message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.SetProperties
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.SetProperties} SetProperties
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                SetProperties.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.SetProperties();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                if (!(message.properties && message.properties.length))
                                    message.properties = [];
                                message.properties.push($root.eyepiece.rpc.v1.Property.decode(reader, reader.uint32(), undefined, long + 1));
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a SetProperties message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.SetProperties
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.SetProperties} SetProperties
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                SetProperties.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a SetProperties message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.SetProperties
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                SetProperties.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.properties != null && Object.hasOwnProperty.call(message, "properties")) {
                        if (!Array.isArray(message.properties))
                            return "properties: array expected";
                        for (let i = 0; i < message.properties.length; ++i) {
                            let error = $root.eyepiece.rpc.v1.Property.verify(message.properties[i], long + 1);
                            if (error)
                                return "properties." + error;
                        }
                    }
                    return null;
                };

                /**
                 * Creates a SetProperties message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.SetProperties
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.SetProperties} SetProperties
                 */
                SetProperties.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.SetProperties)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.SetProperties: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.SetProperties();
                    if (object.properties) {
                        if (!Array.isArray(object.properties))
                            throw TypeError(".eyepiece.rpc.v1.SetProperties.properties: array expected");
                        message.properties = [];
                        for (let i = 0; i < object.properties.length; ++i) {
                            if (!$util.isObject(object.properties[i]))
                                throw TypeError(".eyepiece.rpc.v1.SetProperties.properties: object expected");
                            message.properties[i] = $root.eyepiece.rpc.v1.Property.fromObject(object.properties[i], long + 1);
                        }
                    }
                    return message;
                };

                /**
                 * Creates a plain object from a SetProperties message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.SetProperties
                 * @static
                 * @param {eyepiece.rpc.v1.SetProperties} message SetProperties
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                SetProperties.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.arrays || options.defaults)
                        object.properties = [];
                    if (message.properties && message.properties.length) {
                        object.properties = [];
                        for (let j = 0; j < message.properties.length; ++j)
                            object.properties[j] = $root.eyepiece.rpc.v1.Property.toObject(message.properties[j], options, q + 1);
                    }
                    return object;
                };

                /**
                 * Converts this SetProperties to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.SetProperties
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                SetProperties.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for SetProperties
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.SetProperties
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                SetProperties.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.SetProperties";
                };

                return SetProperties;
            })();

            v1.Start = (function() {

                /**
                 * Properties of a Start.
                 * @memberof eyepiece.rpc.v1
                 * @interface IStart
                 */

                /**
                 * Constructs a new Start.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a Start.
                 * @implements IStart
                 * @constructor
                 * @param {eyepiece.rpc.v1.IStart=} [properties] Properties to set
                 */
                function Start(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Creates a new Start instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.Start
                 * @static
                 * @param {eyepiece.rpc.v1.IStart=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.Start} Start instance
                 */
                Start.create = function create(properties) {
                    return new Start(properties);
                };

                /**
                 * Encodes the specified Start message. Does not implicitly {@link eyepiece.rpc.v1.Start.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.Start
                 * @static
                 * @param {eyepiece.rpc.v1.IStart} message Start message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Start.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    return writer;
                };

                /**
                 * Encodes the specified Start message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Start.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.Start
                 * @static
                 * @param {eyepiece.rpc.v1.IStart} message Start message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Start.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a Start message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.Start
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.Start} Start
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Start.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.Start();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a Start message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.Start
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.Start} Start
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Start.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a Start message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.Start
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                Start.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    return null;
                };

                /**
                 * Creates a Start message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.Start
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.Start} Start
                 */
                Start.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.Start)
                        return object;
                    return new $root.eyepiece.rpc.v1.Start();
                };

                /**
                 * Creates a plain object from a Start message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.Start
                 * @static
                 * @param {eyepiece.rpc.v1.Start} message Start
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                Start.toObject = function toObject() {
                    return {};
                };

                /**
                 * Converts this Start to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.Start
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                Start.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for Start
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.Start
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                Start.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.Start";
                };

                return Start;
            })();

            v1.Stop = (function() {

                /**
                 * Properties of a Stop.
                 * @memberof eyepiece.rpc.v1
                 * @interface IStop
                 */

                /**
                 * Constructs a new Stop.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a Stop.
                 * @implements IStop
                 * @constructor
                 * @param {eyepiece.rpc.v1.IStop=} [properties] Properties to set
                 */
                function Stop(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Creates a new Stop instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.Stop
                 * @static
                 * @param {eyepiece.rpc.v1.IStop=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.Stop} Stop instance
                 */
                Stop.create = function create(properties) {
                    return new Stop(properties);
                };

                /**
                 * Encodes the specified Stop message. Does not implicitly {@link eyepiece.rpc.v1.Stop.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.Stop
                 * @static
                 * @param {eyepiece.rpc.v1.IStop} message Stop message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Stop.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    return writer;
                };

                /**
                 * Encodes the specified Stop message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Stop.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.Stop
                 * @static
                 * @param {eyepiece.rpc.v1.IStop} message Stop message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Stop.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a Stop message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.Stop
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.Stop} Stop
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Stop.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.Stop();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a Stop message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.Stop
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.Stop} Stop
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Stop.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a Stop message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.Stop
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                Stop.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    return null;
                };

                /**
                 * Creates a Stop message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.Stop
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.Stop} Stop
                 */
                Stop.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.Stop)
                        return object;
                    return new $root.eyepiece.rpc.v1.Stop();
                };

                /**
                 * Creates a plain object from a Stop message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.Stop
                 * @static
                 * @param {eyepiece.rpc.v1.Stop} message Stop
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                Stop.toObject = function toObject() {
                    return {};
                };

                /**
                 * Converts this Stop to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.Stop
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                Stop.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for Stop
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.Stop
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                Stop.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.Stop";
                };

                return Stop;
            })();

            v1.Restart = (function() {

                /**
                 * Properties of a Restart.
                 * @memberof eyepiece.rpc.v1
                 * @interface IRestart
                 */

                /**
                 * Constructs a new Restart.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a Restart.
                 * @implements IRestart
                 * @constructor
                 * @param {eyepiece.rpc.v1.IRestart=} [properties] Properties to set
                 */
                function Restart(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Creates a new Restart instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.Restart
                 * @static
                 * @param {eyepiece.rpc.v1.IRestart=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.Restart} Restart instance
                 */
                Restart.create = function create(properties) {
                    return new Restart(properties);
                };

                /**
                 * Encodes the specified Restart message. Does not implicitly {@link eyepiece.rpc.v1.Restart.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.Restart
                 * @static
                 * @param {eyepiece.rpc.v1.IRestart} message Restart message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Restart.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    return writer;
                };

                /**
                 * Encodes the specified Restart message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Restart.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.Restart
                 * @static
                 * @param {eyepiece.rpc.v1.IRestart} message Restart message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Restart.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a Restart message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.Restart
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.Restart} Restart
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Restart.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.Restart();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a Restart message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.Restart
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.Restart} Restart
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Restart.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a Restart message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.Restart
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                Restart.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    return null;
                };

                /**
                 * Creates a Restart message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.Restart
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.Restart} Restart
                 */
                Restart.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.Restart)
                        return object;
                    return new $root.eyepiece.rpc.v1.Restart();
                };

                /**
                 * Creates a plain object from a Restart message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.Restart
                 * @static
                 * @param {eyepiece.rpc.v1.Restart} message Restart
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                Restart.toObject = function toObject() {
                    return {};
                };

                /**
                 * Converts this Restart to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.Restart
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                Restart.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for Restart
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.Restart
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                Restart.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.Restart";
                };

                return Restart;
            })();

            v1.Recenter = (function() {

                /**
                 * Properties of a Recenter.
                 * @memberof eyepiece.rpc.v1
                 * @interface IRecenter
                 */

                /**
                 * Constructs a new Recenter.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a Recenter.
                 * @implements IRecenter
                 * @constructor
                 * @param {eyepiece.rpc.v1.IRecenter=} [properties] Properties to set
                 */
                function Recenter(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Creates a new Recenter instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.Recenter
                 * @static
                 * @param {eyepiece.rpc.v1.IRecenter=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.Recenter} Recenter instance
                 */
                Recenter.create = function create(properties) {
                    return new Recenter(properties);
                };

                /**
                 * Encodes the specified Recenter message. Does not implicitly {@link eyepiece.rpc.v1.Recenter.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.Recenter
                 * @static
                 * @param {eyepiece.rpc.v1.IRecenter} message Recenter message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Recenter.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    return writer;
                };

                /**
                 * Encodes the specified Recenter message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Recenter.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.Recenter
                 * @static
                 * @param {eyepiece.rpc.v1.IRecenter} message Recenter message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Recenter.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a Recenter message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.Recenter
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.Recenter} Recenter
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Recenter.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.Recenter();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a Recenter message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.Recenter
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.Recenter} Recenter
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Recenter.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a Recenter message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.Recenter
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                Recenter.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    return null;
                };

                /**
                 * Creates a Recenter message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.Recenter
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.Recenter} Recenter
                 */
                Recenter.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.Recenter)
                        return object;
                    return new $root.eyepiece.rpc.v1.Recenter();
                };

                /**
                 * Creates a plain object from a Recenter message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.Recenter
                 * @static
                 * @param {eyepiece.rpc.v1.Recenter} message Recenter
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                Recenter.toObject = function toObject() {
                    return {};
                };

                /**
                 * Converts this Recenter to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.Recenter
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                Recenter.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for Recenter
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.Recenter
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                Recenter.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.Recenter";
                };

                return Recenter;
            })();

            v1.ForceIdr = (function() {

                /**
                 * Properties of a ForceIdr.
                 * @memberof eyepiece.rpc.v1
                 * @interface IForceIdr
                 */

                /**
                 * Constructs a new ForceIdr.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a ForceIdr.
                 * @implements IForceIdr
                 * @constructor
                 * @param {eyepiece.rpc.v1.IForceIdr=} [properties] Properties to set
                 */
                function ForceIdr(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Creates a new ForceIdr instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.ForceIdr
                 * @static
                 * @param {eyepiece.rpc.v1.IForceIdr=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.ForceIdr} ForceIdr instance
                 */
                ForceIdr.create = function create(properties) {
                    return new ForceIdr(properties);
                };

                /**
                 * Encodes the specified ForceIdr message. Does not implicitly {@link eyepiece.rpc.v1.ForceIdr.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.ForceIdr
                 * @static
                 * @param {eyepiece.rpc.v1.IForceIdr} message ForceIdr message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                ForceIdr.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    return writer;
                };

                /**
                 * Encodes the specified ForceIdr message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.ForceIdr.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.ForceIdr
                 * @static
                 * @param {eyepiece.rpc.v1.IForceIdr} message ForceIdr message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                ForceIdr.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a ForceIdr message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.ForceIdr
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.ForceIdr} ForceIdr
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                ForceIdr.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.ForceIdr();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a ForceIdr message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.ForceIdr
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.ForceIdr} ForceIdr
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                ForceIdr.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a ForceIdr message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.ForceIdr
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                ForceIdr.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    return null;
                };

                /**
                 * Creates a ForceIdr message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.ForceIdr
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.ForceIdr} ForceIdr
                 */
                ForceIdr.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.ForceIdr)
                        return object;
                    return new $root.eyepiece.rpc.v1.ForceIdr();
                };

                /**
                 * Creates a plain object from a ForceIdr message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.ForceIdr
                 * @static
                 * @param {eyepiece.rpc.v1.ForceIdr} message ForceIdr
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                ForceIdr.toObject = function toObject() {
                    return {};
                };

                /**
                 * Converts this ForceIdr to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.ForceIdr
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                ForceIdr.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for ForceIdr
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.ForceIdr
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                ForceIdr.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.ForceIdr";
                };

                return ForceIdr;
            })();

            v1.SetPreview = (function() {

                /**
                 * Properties of a SetPreview.
                 * @memberof eyepiece.rpc.v1
                 * @interface ISetPreview
                 * @property {boolean|null} [enabled] SetPreview enabled
                 * @property {Array.<string>|null} [supportedCodecs] SetPreview supportedCodecs
                 * @property {boolean|null} [keyframesOnly] SetPreview keyframesOnly
                 */

                /**
                 * Constructs a new SetPreview.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a SetPreview.
                 * @implements ISetPreview
                 * @constructor
                 * @param {eyepiece.rpc.v1.ISetPreview=} [properties] Properties to set
                 */
                function SetPreview(properties) {
                    this.supportedCodecs = [];
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * SetPreview enabled.
                 * @member {boolean} enabled
                 * @memberof eyepiece.rpc.v1.SetPreview
                 * @instance
                 */
                SetPreview.prototype.enabled = false;

                /**
                 * SetPreview supportedCodecs.
                 * @member {Array.<string>} supportedCodecs
                 * @memberof eyepiece.rpc.v1.SetPreview
                 * @instance
                 */
                SetPreview.prototype.supportedCodecs = $util.emptyArray;

                /**
                 * SetPreview keyframesOnly.
                 * @member {boolean} keyframesOnly
                 * @memberof eyepiece.rpc.v1.SetPreview
                 * @instance
                 */
                SetPreview.prototype.keyframesOnly = false;

                /**
                 * Creates a new SetPreview instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.SetPreview
                 * @static
                 * @param {eyepiece.rpc.v1.ISetPreview=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.SetPreview} SetPreview instance
                 */
                SetPreview.create = function create(properties) {
                    return new SetPreview(properties);
                };

                /**
                 * Encodes the specified SetPreview message. Does not implicitly {@link eyepiece.rpc.v1.SetPreview.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.SetPreview
                 * @static
                 * @param {eyepiece.rpc.v1.ISetPreview} message SetPreview message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                SetPreview.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.enabled != null && Object.hasOwnProperty.call(message, "enabled"))
                        writer.uint32(/* id 1, wireType 0 =*/8).bool(message.enabled);
                    if (message.supportedCodecs != null && message.supportedCodecs.length)
                        for (let i = 0; i < message.supportedCodecs.length; ++i)
                            writer.uint32(/* id 2, wireType 2 =*/18).string(message.supportedCodecs[i]);
                    if (message.keyframesOnly != null && Object.hasOwnProperty.call(message, "keyframesOnly"))
                        writer.uint32(/* id 3, wireType 0 =*/24).bool(message.keyframesOnly);
                    return writer;
                };

                /**
                 * Encodes the specified SetPreview message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.SetPreview.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.SetPreview
                 * @static
                 * @param {eyepiece.rpc.v1.ISetPreview} message SetPreview message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                SetPreview.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a SetPreview message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.SetPreview
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.SetPreview} SetPreview
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                SetPreview.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.SetPreview();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                message.enabled = reader.bool();
                                break;
                            }
                        case 2: {
                                if (!(message.supportedCodecs && message.supportedCodecs.length))
                                    message.supportedCodecs = [];
                                message.supportedCodecs.push(reader.string());
                                break;
                            }
                        case 3: {
                                message.keyframesOnly = reader.bool();
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a SetPreview message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.SetPreview
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.SetPreview} SetPreview
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                SetPreview.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a SetPreview message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.SetPreview
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                SetPreview.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.enabled != null && Object.hasOwnProperty.call(message, "enabled"))
                        if (typeof message.enabled !== "boolean")
                            return "enabled: boolean expected";
                    if (message.supportedCodecs != null && Object.hasOwnProperty.call(message, "supportedCodecs")) {
                        if (!Array.isArray(message.supportedCodecs))
                            return "supportedCodecs: array expected";
                        for (let i = 0; i < message.supportedCodecs.length; ++i)
                            if (!$util.isString(message.supportedCodecs[i]))
                                return "supportedCodecs: string[] expected";
                    }
                    if (message.keyframesOnly != null && Object.hasOwnProperty.call(message, "keyframesOnly"))
                        if (typeof message.keyframesOnly !== "boolean")
                            return "keyframesOnly: boolean expected";
                    return null;
                };

                /**
                 * Creates a SetPreview message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.SetPreview
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.SetPreview} SetPreview
                 */
                SetPreview.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.SetPreview)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.SetPreview: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.SetPreview();
                    if (object.enabled != null)
                        message.enabled = Boolean(object.enabled);
                    if (object.supportedCodecs) {
                        if (!Array.isArray(object.supportedCodecs))
                            throw TypeError(".eyepiece.rpc.v1.SetPreview.supportedCodecs: array expected");
                        message.supportedCodecs = [];
                        for (let i = 0; i < object.supportedCodecs.length; ++i)
                            message.supportedCodecs[i] = String(object.supportedCodecs[i]);
                    }
                    if (object.keyframesOnly != null)
                        message.keyframesOnly = Boolean(object.keyframesOnly);
                    return message;
                };

                /**
                 * Creates a plain object from a SetPreview message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.SetPreview
                 * @static
                 * @param {eyepiece.rpc.v1.SetPreview} message SetPreview
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                SetPreview.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.arrays || options.defaults)
                        object.supportedCodecs = [];
                    if (options.defaults) {
                        object.enabled = false;
                        object.keyframesOnly = false;
                    }
                    if (message.enabled != null && Object.hasOwnProperty.call(message, "enabled"))
                        object.enabled = message.enabled;
                    if (message.supportedCodecs && message.supportedCodecs.length) {
                        object.supportedCodecs = [];
                        for (let j = 0; j < message.supportedCodecs.length; ++j)
                            object.supportedCodecs[j] = message.supportedCodecs[j];
                    }
                    if (message.keyframesOnly != null && Object.hasOwnProperty.call(message, "keyframesOnly"))
                        object.keyframesOnly = message.keyframesOnly;
                    return object;
                };

                /**
                 * Converts this SetPreview to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.SetPreview
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                SetPreview.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for SetPreview
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.SetPreview
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                SetPreview.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.SetPreview";
                };

                return SetPreview;
            })();

            v1.SetCommandLine = (function() {

                /**
                 * Properties of a SetCommandLine.
                 * @memberof eyepiece.rpc.v1
                 * @interface ISetCommandLine
                 * @property {string|null} [commandLine] SetCommandLine commandLine
                 */

                /**
                 * Constructs a new SetCommandLine.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a SetCommandLine.
                 * @implements ISetCommandLine
                 * @constructor
                 * @param {eyepiece.rpc.v1.ISetCommandLine=} [properties] Properties to set
                 */
                function SetCommandLine(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * SetCommandLine commandLine.
                 * @member {string} commandLine
                 * @memberof eyepiece.rpc.v1.SetCommandLine
                 * @instance
                 */
                SetCommandLine.prototype.commandLine = "";

                /**
                 * Creates a new SetCommandLine instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.SetCommandLine
                 * @static
                 * @param {eyepiece.rpc.v1.ISetCommandLine=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.SetCommandLine} SetCommandLine instance
                 */
                SetCommandLine.create = function create(properties) {
                    return new SetCommandLine(properties);
                };

                /**
                 * Encodes the specified SetCommandLine message. Does not implicitly {@link eyepiece.rpc.v1.SetCommandLine.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.SetCommandLine
                 * @static
                 * @param {eyepiece.rpc.v1.ISetCommandLine} message SetCommandLine message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                SetCommandLine.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.commandLine != null && Object.hasOwnProperty.call(message, "commandLine"))
                        writer.uint32(/* id 1, wireType 2 =*/10).string(message.commandLine);
                    return writer;
                };

                /**
                 * Encodes the specified SetCommandLine message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.SetCommandLine.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.SetCommandLine
                 * @static
                 * @param {eyepiece.rpc.v1.ISetCommandLine} message SetCommandLine message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                SetCommandLine.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a SetCommandLine message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.SetCommandLine
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.SetCommandLine} SetCommandLine
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                SetCommandLine.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.SetCommandLine();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                message.commandLine = reader.string();
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a SetCommandLine message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.SetCommandLine
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.SetCommandLine} SetCommandLine
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                SetCommandLine.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a SetCommandLine message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.SetCommandLine
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                SetCommandLine.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.commandLine != null && Object.hasOwnProperty.call(message, "commandLine"))
                        if (!$util.isString(message.commandLine))
                            return "commandLine: string expected";
                    return null;
                };

                /**
                 * Creates a SetCommandLine message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.SetCommandLine
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.SetCommandLine} SetCommandLine
                 */
                SetCommandLine.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.SetCommandLine)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.SetCommandLine: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.SetCommandLine();
                    if (object.commandLine != null)
                        message.commandLine = String(object.commandLine);
                    return message;
                };

                /**
                 * Creates a plain object from a SetCommandLine message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.SetCommandLine
                 * @static
                 * @param {eyepiece.rpc.v1.SetCommandLine} message SetCommandLine
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                SetCommandLine.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.defaults)
                        object.commandLine = "";
                    if (message.commandLine != null && Object.hasOwnProperty.call(message, "commandLine"))
                        object.commandLine = message.commandLine;
                    return object;
                };

                /**
                 * Converts this SetCommandLine to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.SetCommandLine
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                SetCommandLine.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for SetCommandLine
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.SetCommandLine
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                SetCommandLine.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.SetCommandLine";
                };

                return SetCommandLine;
            })();

            v1.ListKmsOutputs = (function() {

                /**
                 * Properties of a ListKmsOutputs.
                 * @memberof eyepiece.rpc.v1
                 * @interface IListKmsOutputs
                 */

                /**
                 * Constructs a new ListKmsOutputs.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a ListKmsOutputs.
                 * @implements IListKmsOutputs
                 * @constructor
                 * @param {eyepiece.rpc.v1.IListKmsOutputs=} [properties] Properties to set
                 */
                function ListKmsOutputs(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Creates a new ListKmsOutputs instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.ListKmsOutputs
                 * @static
                 * @param {eyepiece.rpc.v1.IListKmsOutputs=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.ListKmsOutputs} ListKmsOutputs instance
                 */
                ListKmsOutputs.create = function create(properties) {
                    return new ListKmsOutputs(properties);
                };

                /**
                 * Encodes the specified ListKmsOutputs message. Does not implicitly {@link eyepiece.rpc.v1.ListKmsOutputs.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.ListKmsOutputs
                 * @static
                 * @param {eyepiece.rpc.v1.IListKmsOutputs} message ListKmsOutputs message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                ListKmsOutputs.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    return writer;
                };

                /**
                 * Encodes the specified ListKmsOutputs message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.ListKmsOutputs.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.ListKmsOutputs
                 * @static
                 * @param {eyepiece.rpc.v1.IListKmsOutputs} message ListKmsOutputs message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                ListKmsOutputs.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a ListKmsOutputs message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.ListKmsOutputs
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.ListKmsOutputs} ListKmsOutputs
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                ListKmsOutputs.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.ListKmsOutputs();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a ListKmsOutputs message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.ListKmsOutputs
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.ListKmsOutputs} ListKmsOutputs
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                ListKmsOutputs.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a ListKmsOutputs message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.ListKmsOutputs
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                ListKmsOutputs.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    return null;
                };

                /**
                 * Creates a ListKmsOutputs message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.ListKmsOutputs
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.ListKmsOutputs} ListKmsOutputs
                 */
                ListKmsOutputs.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.ListKmsOutputs)
                        return object;
                    return new $root.eyepiece.rpc.v1.ListKmsOutputs();
                };

                /**
                 * Creates a plain object from a ListKmsOutputs message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.ListKmsOutputs
                 * @static
                 * @param {eyepiece.rpc.v1.ListKmsOutputs} message ListKmsOutputs
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                ListKmsOutputs.toObject = function toObject() {
                    return {};
                };

                /**
                 * Converts this ListKmsOutputs to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.ListKmsOutputs
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                ListKmsOutputs.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for ListKmsOutputs
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.ListKmsOutputs
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                ListKmsOutputs.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.ListKmsOutputs";
                };

                return ListKmsOutputs;
            })();

            v1.State = (function() {

                /**
                 * Properties of a State.
                 * @memberof eyepiece.rpc.v1
                 * @interface IState
                 * @property {string|null} [version] State version
                 * @property {string|null} [settingsSchemaJson] State settingsSchemaJson
                 * @property {Array.<eyepiece.rpc.v1.IProperty>|null} [properties] State properties
                 * @property {string|null} [phase] State phase
                 * @property {boolean|null} [desiredRunning] State desiredRunning
                 * @property {boolean|null} [connected] State connected
                 * @property {string|null} [commandLine] State commandLine
                 * @property {string|null} [runningCommandLine] State runningCommandLine
                 * @property {Array.<eyepiece.rpc.v1.IProperty>|null} [runningProperties] State runningProperties
                 */

                /**
                 * Constructs a new State.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a State.
                 * @implements IState
                 * @constructor
                 * @param {eyepiece.rpc.v1.IState=} [properties] Properties to set
                 */
                function State(properties) {
                    this.properties = [];
                    this.runningProperties = [];
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * State version.
                 * @member {string} version
                 * @memberof eyepiece.rpc.v1.State
                 * @instance
                 */
                State.prototype.version = "";

                /**
                 * State settingsSchemaJson.
                 * @member {string} settingsSchemaJson
                 * @memberof eyepiece.rpc.v1.State
                 * @instance
                 */
                State.prototype.settingsSchemaJson = "";

                /**
                 * State properties.
                 * @member {Array.<eyepiece.rpc.v1.IProperty>} properties
                 * @memberof eyepiece.rpc.v1.State
                 * @instance
                 */
                State.prototype.properties = $util.emptyArray;

                /**
                 * State phase.
                 * @member {string} phase
                 * @memberof eyepiece.rpc.v1.State
                 * @instance
                 */
                State.prototype.phase = "";

                /**
                 * State desiredRunning.
                 * @member {boolean} desiredRunning
                 * @memberof eyepiece.rpc.v1.State
                 * @instance
                 */
                State.prototype.desiredRunning = false;

                /**
                 * State connected.
                 * @member {boolean} connected
                 * @memberof eyepiece.rpc.v1.State
                 * @instance
                 */
                State.prototype.connected = false;

                /**
                 * State commandLine.
                 * @member {string} commandLine
                 * @memberof eyepiece.rpc.v1.State
                 * @instance
                 */
                State.prototype.commandLine = "";

                /**
                 * State runningCommandLine.
                 * @member {string} runningCommandLine
                 * @memberof eyepiece.rpc.v1.State
                 * @instance
                 */
                State.prototype.runningCommandLine = "";

                /**
                 * State runningProperties.
                 * @member {Array.<eyepiece.rpc.v1.IProperty>} runningProperties
                 * @memberof eyepiece.rpc.v1.State
                 * @instance
                 */
                State.prototype.runningProperties = $util.emptyArray;

                /**
                 * Creates a new State instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.State
                 * @static
                 * @param {eyepiece.rpc.v1.IState=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.State} State instance
                 */
                State.create = function create(properties) {
                    return new State(properties);
                };

                /**
                 * Encodes the specified State message. Does not implicitly {@link eyepiece.rpc.v1.State.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.State
                 * @static
                 * @param {eyepiece.rpc.v1.IState} message State message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                State.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.version != null && Object.hasOwnProperty.call(message, "version"))
                        writer.uint32(/* id 1, wireType 2 =*/10).string(message.version);
                    if (message.settingsSchemaJson != null && Object.hasOwnProperty.call(message, "settingsSchemaJson"))
                        writer.uint32(/* id 2, wireType 2 =*/18).string(message.settingsSchemaJson);
                    if (message.properties != null && message.properties.length)
                        for (let i = 0; i < message.properties.length; ++i)
                            $root.eyepiece.rpc.v1.Property.encode(message.properties[i], writer.uint32(/* id 3, wireType 2 =*/26).fork(), q + 1).ldelim();
                    if (message.phase != null && Object.hasOwnProperty.call(message, "phase"))
                        writer.uint32(/* id 4, wireType 2 =*/34).string(message.phase);
                    if (message.desiredRunning != null && Object.hasOwnProperty.call(message, "desiredRunning"))
                        writer.uint32(/* id 5, wireType 0 =*/40).bool(message.desiredRunning);
                    if (message.connected != null && Object.hasOwnProperty.call(message, "connected"))
                        writer.uint32(/* id 6, wireType 0 =*/48).bool(message.connected);
                    if (message.commandLine != null && Object.hasOwnProperty.call(message, "commandLine"))
                        writer.uint32(/* id 7, wireType 2 =*/58).string(message.commandLine);
                    if (message.runningCommandLine != null && Object.hasOwnProperty.call(message, "runningCommandLine"))
                        writer.uint32(/* id 8, wireType 2 =*/66).string(message.runningCommandLine);
                    if (message.runningProperties != null && message.runningProperties.length)
                        for (let i = 0; i < message.runningProperties.length; ++i)
                            $root.eyepiece.rpc.v1.Property.encode(message.runningProperties[i], writer.uint32(/* id 9, wireType 2 =*/74).fork(), q + 1).ldelim();
                    return writer;
                };

                /**
                 * Encodes the specified State message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.State.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.State
                 * @static
                 * @param {eyepiece.rpc.v1.IState} message State message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                State.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a State message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.State
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.State} State
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                State.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.State();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                message.version = reader.string();
                                break;
                            }
                        case 2: {
                                message.settingsSchemaJson = reader.string();
                                break;
                            }
                        case 3: {
                                if (!(message.properties && message.properties.length))
                                    message.properties = [];
                                message.properties.push($root.eyepiece.rpc.v1.Property.decode(reader, reader.uint32(), undefined, long + 1));
                                break;
                            }
                        case 4: {
                                message.phase = reader.string();
                                break;
                            }
                        case 5: {
                                message.desiredRunning = reader.bool();
                                break;
                            }
                        case 6: {
                                message.connected = reader.bool();
                                break;
                            }
                        case 7: {
                                message.commandLine = reader.string();
                                break;
                            }
                        case 8: {
                                message.runningCommandLine = reader.string();
                                break;
                            }
                        case 9: {
                                if (!(message.runningProperties && message.runningProperties.length))
                                    message.runningProperties = [];
                                message.runningProperties.push($root.eyepiece.rpc.v1.Property.decode(reader, reader.uint32(), undefined, long + 1));
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a State message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.State
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.State} State
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                State.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a State message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.State
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                State.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.version != null && Object.hasOwnProperty.call(message, "version"))
                        if (!$util.isString(message.version))
                            return "version: string expected";
                    if (message.settingsSchemaJson != null && Object.hasOwnProperty.call(message, "settingsSchemaJson"))
                        if (!$util.isString(message.settingsSchemaJson))
                            return "settingsSchemaJson: string expected";
                    if (message.properties != null && Object.hasOwnProperty.call(message, "properties")) {
                        if (!Array.isArray(message.properties))
                            return "properties: array expected";
                        for (let i = 0; i < message.properties.length; ++i) {
                            let error = $root.eyepiece.rpc.v1.Property.verify(message.properties[i], long + 1);
                            if (error)
                                return "properties." + error;
                        }
                    }
                    if (message.phase != null && Object.hasOwnProperty.call(message, "phase"))
                        if (!$util.isString(message.phase))
                            return "phase: string expected";
                    if (message.desiredRunning != null && Object.hasOwnProperty.call(message, "desiredRunning"))
                        if (typeof message.desiredRunning !== "boolean")
                            return "desiredRunning: boolean expected";
                    if (message.connected != null && Object.hasOwnProperty.call(message, "connected"))
                        if (typeof message.connected !== "boolean")
                            return "connected: boolean expected";
                    if (message.commandLine != null && Object.hasOwnProperty.call(message, "commandLine"))
                        if (!$util.isString(message.commandLine))
                            return "commandLine: string expected";
                    if (message.runningCommandLine != null && Object.hasOwnProperty.call(message, "runningCommandLine"))
                        if (!$util.isString(message.runningCommandLine))
                            return "runningCommandLine: string expected";
                    if (message.runningProperties != null && Object.hasOwnProperty.call(message, "runningProperties")) {
                        if (!Array.isArray(message.runningProperties))
                            return "runningProperties: array expected";
                        for (let i = 0; i < message.runningProperties.length; ++i) {
                            let error = $root.eyepiece.rpc.v1.Property.verify(message.runningProperties[i], long + 1);
                            if (error)
                                return "runningProperties." + error;
                        }
                    }
                    return null;
                };

                /**
                 * Creates a State message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.State
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.State} State
                 */
                State.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.State)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.State: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.State();
                    if (object.version != null)
                        message.version = String(object.version);
                    if (object.settingsSchemaJson != null)
                        message.settingsSchemaJson = String(object.settingsSchemaJson);
                    if (object.properties) {
                        if (!Array.isArray(object.properties))
                            throw TypeError(".eyepiece.rpc.v1.State.properties: array expected");
                        message.properties = [];
                        for (let i = 0; i < object.properties.length; ++i) {
                            if (!$util.isObject(object.properties[i]))
                                throw TypeError(".eyepiece.rpc.v1.State.properties: object expected");
                            message.properties[i] = $root.eyepiece.rpc.v1.Property.fromObject(object.properties[i], long + 1);
                        }
                    }
                    if (object.phase != null)
                        message.phase = String(object.phase);
                    if (object.desiredRunning != null)
                        message.desiredRunning = Boolean(object.desiredRunning);
                    if (object.connected != null)
                        message.connected = Boolean(object.connected);
                    if (object.commandLine != null)
                        message.commandLine = String(object.commandLine);
                    if (object.runningCommandLine != null)
                        message.runningCommandLine = String(object.runningCommandLine);
                    if (object.runningProperties) {
                        if (!Array.isArray(object.runningProperties))
                            throw TypeError(".eyepiece.rpc.v1.State.runningProperties: array expected");
                        message.runningProperties = [];
                        for (let i = 0; i < object.runningProperties.length; ++i) {
                            if (!$util.isObject(object.runningProperties[i]))
                                throw TypeError(".eyepiece.rpc.v1.State.runningProperties: object expected");
                            message.runningProperties[i] = $root.eyepiece.rpc.v1.Property.fromObject(object.runningProperties[i], long + 1);
                        }
                    }
                    return message;
                };

                /**
                 * Creates a plain object from a State message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.State
                 * @static
                 * @param {eyepiece.rpc.v1.State} message State
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                State.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.arrays || options.defaults) {
                        object.properties = [];
                        object.runningProperties = [];
                    }
                    if (options.defaults) {
                        object.version = "";
                        object.settingsSchemaJson = "";
                        object.phase = "";
                        object.desiredRunning = false;
                        object.connected = false;
                        object.commandLine = "";
                        object.runningCommandLine = "";
                    }
                    if (message.version != null && Object.hasOwnProperty.call(message, "version"))
                        object.version = message.version;
                    if (message.settingsSchemaJson != null && Object.hasOwnProperty.call(message, "settingsSchemaJson"))
                        object.settingsSchemaJson = message.settingsSchemaJson;
                    if (message.properties && message.properties.length) {
                        object.properties = [];
                        for (let j = 0; j < message.properties.length; ++j)
                            object.properties[j] = $root.eyepiece.rpc.v1.Property.toObject(message.properties[j], options, q + 1);
                    }
                    if (message.phase != null && Object.hasOwnProperty.call(message, "phase"))
                        object.phase = message.phase;
                    if (message.desiredRunning != null && Object.hasOwnProperty.call(message, "desiredRunning"))
                        object.desiredRunning = message.desiredRunning;
                    if (message.connected != null && Object.hasOwnProperty.call(message, "connected"))
                        object.connected = message.connected;
                    if (message.commandLine != null && Object.hasOwnProperty.call(message, "commandLine"))
                        object.commandLine = message.commandLine;
                    if (message.runningCommandLine != null && Object.hasOwnProperty.call(message, "runningCommandLine"))
                        object.runningCommandLine = message.runningCommandLine;
                    if (message.runningProperties && message.runningProperties.length) {
                        object.runningProperties = [];
                        for (let j = 0; j < message.runningProperties.length; ++j)
                            object.runningProperties[j] = $root.eyepiece.rpc.v1.Property.toObject(message.runningProperties[j], options, q + 1);
                    }
                    return object;
                };

                /**
                 * Converts this State to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.State
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                State.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for State
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.State
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                State.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.State";
                };

                return State;
            })();

            v1.Error = (function() {

                /**
                 * Properties of an Error.
                 * @memberof eyepiece.rpc.v1
                 * @interface IError
                 * @property {string|null} [message] Error message
                 */

                /**
                 * Constructs a new Error.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents an Error.
                 * @implements IError
                 * @constructor
                 * @param {eyepiece.rpc.v1.IError=} [properties] Properties to set
                 */
                function Error(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Error message.
                 * @member {string} message
                 * @memberof eyepiece.rpc.v1.Error
                 * @instance
                 */
                Error.prototype.message = "";

                /**
                 * Creates a new Error instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.Error
                 * @static
                 * @param {eyepiece.rpc.v1.IError=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.Error} Error instance
                 */
                Error.create = function create(properties) {
                    return new Error(properties);
                };

                /**
                 * Encodes the specified Error message. Does not implicitly {@link eyepiece.rpc.v1.Error.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.Error
                 * @static
                 * @param {eyepiece.rpc.v1.IError} message Error message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Error.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.message != null && Object.hasOwnProperty.call(message, "message"))
                        writer.uint32(/* id 1, wireType 2 =*/10).string(message.message);
                    return writer;
                };

                /**
                 * Encodes the specified Error message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Error.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.Error
                 * @static
                 * @param {eyepiece.rpc.v1.IError} message Error message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Error.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes an Error message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.Error
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.Error} Error
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Error.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.Error();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                message.message = reader.string();
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes an Error message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.Error
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.Error} Error
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Error.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies an Error message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.Error
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                Error.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.message != null && Object.hasOwnProperty.call(message, "message"))
                        if (!$util.isString(message.message))
                            return "message: string expected";
                    return null;
                };

                /**
                 * Creates an Error message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.Error
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.Error} Error
                 */
                Error.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.Error)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.Error: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.Error();
                    if (object.message != null)
                        message.message = String(object.message);
                    return message;
                };

                /**
                 * Creates a plain object from an Error message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.Error
                 * @static
                 * @param {eyepiece.rpc.v1.Error} message Error
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                Error.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.defaults)
                        object.message = "";
                    if (message.message != null && Object.hasOwnProperty.call(message, "message"))
                        object.message = message.message;
                    return object;
                };

                /**
                 * Converts this Error to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.Error
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                Error.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for Error
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.Error
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                Error.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.Error";
                };

                return Error;
            })();

            v1.Log = (function() {

                /**
                 * Properties of a Log.
                 * @memberof eyepiece.rpc.v1
                 * @interface ILog
                 * @property {string|null} [line] Log line
                 */

                /**
                 * Constructs a new Log.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a Log.
                 * @implements ILog
                 * @constructor
                 * @param {eyepiece.rpc.v1.ILog=} [properties] Properties to set
                 */
                function Log(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Log line.
                 * @member {string} line
                 * @memberof eyepiece.rpc.v1.Log
                 * @instance
                 */
                Log.prototype.line = "";

                /**
                 * Creates a new Log instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.Log
                 * @static
                 * @param {eyepiece.rpc.v1.ILog=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.Log} Log instance
                 */
                Log.create = function create(properties) {
                    return new Log(properties);
                };

                /**
                 * Encodes the specified Log message. Does not implicitly {@link eyepiece.rpc.v1.Log.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.Log
                 * @static
                 * @param {eyepiece.rpc.v1.ILog} message Log message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Log.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.line != null && Object.hasOwnProperty.call(message, "line"))
                        writer.uint32(/* id 1, wireType 2 =*/10).string(message.line);
                    return writer;
                };

                /**
                 * Encodes the specified Log message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Log.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.Log
                 * @static
                 * @param {eyepiece.rpc.v1.ILog} message Log message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Log.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a Log message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.Log
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.Log} Log
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Log.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.Log();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                message.line = reader.string();
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a Log message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.Log
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.Log} Log
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Log.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a Log message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.Log
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                Log.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.line != null && Object.hasOwnProperty.call(message, "line"))
                        if (!$util.isString(message.line))
                            return "line: string expected";
                    return null;
                };

                /**
                 * Creates a Log message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.Log
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.Log} Log
                 */
                Log.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.Log)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.Log: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.Log();
                    if (object.line != null)
                        message.line = String(object.line);
                    return message;
                };

                /**
                 * Creates a plain object from a Log message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.Log
                 * @static
                 * @param {eyepiece.rpc.v1.Log} message Log
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                Log.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.defaults)
                        object.line = "";
                    if (message.line != null && Object.hasOwnProperty.call(message, "line"))
                        object.line = message.line;
                    return object;
                };

                /**
                 * Converts this Log to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.Log
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                Log.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for Log
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.Log
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                Log.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.Log";
                };

                return Log;
            })();

            v1.Pose = (function() {

                /**
                 * Properties of a Pose.
                 * @memberof eyepiece.rpc.v1
                 * @interface IPose
                 * @property {number|null} [yaw] Pose yaw
                 * @property {number|null} [pitch] Pose pitch
                 * @property {number|null} [roll] Pose roll
                 * @property {number|Long|null} [deviceUs] Pose deviceUs
                 * @property {boolean|null} [active] Pose active
                 * @property {number|null} [source] Pose source
                 */

                /**
                 * Constructs a new Pose.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a Pose.
                 * @implements IPose
                 * @constructor
                 * @param {eyepiece.rpc.v1.IPose=} [properties] Properties to set
                 */
                function Pose(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Pose yaw.
                 * @member {number} yaw
                 * @memberof eyepiece.rpc.v1.Pose
                 * @instance
                 */
                Pose.prototype.yaw = 0;

                /**
                 * Pose pitch.
                 * @member {number} pitch
                 * @memberof eyepiece.rpc.v1.Pose
                 * @instance
                 */
                Pose.prototype.pitch = 0;

                /**
                 * Pose roll.
                 * @member {number} roll
                 * @memberof eyepiece.rpc.v1.Pose
                 * @instance
                 */
                Pose.prototype.roll = 0;

                /**
                 * Pose deviceUs.
                 * @member {number|Long} deviceUs
                 * @memberof eyepiece.rpc.v1.Pose
                 * @instance
                 */
                Pose.prototype.deviceUs = $util.Long ? $util.Long.fromBits(0,0,true) : 0;

                /**
                 * Pose active.
                 * @member {boolean} active
                 * @memberof eyepiece.rpc.v1.Pose
                 * @instance
                 */
                Pose.prototype.active = false;

                /**
                 * Pose source.
                 * @member {number} source
                 * @memberof eyepiece.rpc.v1.Pose
                 * @instance
                 */
                Pose.prototype.source = 0;

                /**
                 * Creates a new Pose instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.Pose
                 * @static
                 * @param {eyepiece.rpc.v1.IPose=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.Pose} Pose instance
                 */
                Pose.create = function create(properties) {
                    return new Pose(properties);
                };

                /**
                 * Encodes the specified Pose message. Does not implicitly {@link eyepiece.rpc.v1.Pose.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.Pose
                 * @static
                 * @param {eyepiece.rpc.v1.IPose} message Pose message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Pose.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.yaw != null && Object.hasOwnProperty.call(message, "yaw"))
                        writer.uint32(/* id 1, wireType 5 =*/13).float(message.yaw);
                    if (message.pitch != null && Object.hasOwnProperty.call(message, "pitch"))
                        writer.uint32(/* id 2, wireType 5 =*/21).float(message.pitch);
                    if (message.roll != null && Object.hasOwnProperty.call(message, "roll"))
                        writer.uint32(/* id 3, wireType 5 =*/29).float(message.roll);
                    if (message.deviceUs != null && Object.hasOwnProperty.call(message, "deviceUs"))
                        writer.uint32(/* id 4, wireType 0 =*/32).uint64(message.deviceUs);
                    if (message.active != null && Object.hasOwnProperty.call(message, "active"))
                        writer.uint32(/* id 5, wireType 0 =*/40).bool(message.active);
                    if (message.source != null && Object.hasOwnProperty.call(message, "source"))
                        writer.uint32(/* id 6, wireType 0 =*/48).uint32(message.source);
                    return writer;
                };

                /**
                 * Encodes the specified Pose message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Pose.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.Pose
                 * @static
                 * @param {eyepiece.rpc.v1.IPose} message Pose message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Pose.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a Pose message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.Pose
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.Pose} Pose
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Pose.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.Pose();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                message.yaw = reader.float();
                                break;
                            }
                        case 2: {
                                message.pitch = reader.float();
                                break;
                            }
                        case 3: {
                                message.roll = reader.float();
                                break;
                            }
                        case 4: {
                                message.deviceUs = reader.uint64();
                                break;
                            }
                        case 5: {
                                message.active = reader.bool();
                                break;
                            }
                        case 6: {
                                message.source = reader.uint32();
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a Pose message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.Pose
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.Pose} Pose
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Pose.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a Pose message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.Pose
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                Pose.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.yaw != null && Object.hasOwnProperty.call(message, "yaw"))
                        if (typeof message.yaw !== "number")
                            return "yaw: number expected";
                    if (message.pitch != null && Object.hasOwnProperty.call(message, "pitch"))
                        if (typeof message.pitch !== "number")
                            return "pitch: number expected";
                    if (message.roll != null && Object.hasOwnProperty.call(message, "roll"))
                        if (typeof message.roll !== "number")
                            return "roll: number expected";
                    if (message.deviceUs != null && Object.hasOwnProperty.call(message, "deviceUs"))
                        if (!$util.isInteger(message.deviceUs) && !(message.deviceUs && $util.isInteger(message.deviceUs.low) && $util.isInteger(message.deviceUs.high)))
                            return "deviceUs: integer|Long expected";
                    if (message.active != null && Object.hasOwnProperty.call(message, "active"))
                        if (typeof message.active !== "boolean")
                            return "active: boolean expected";
                    if (message.source != null && Object.hasOwnProperty.call(message, "source"))
                        if (!$util.isInteger(message.source))
                            return "source: integer expected";
                    return null;
                };

                /**
                 * Creates a Pose message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.Pose
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.Pose} Pose
                 */
                Pose.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.Pose)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.Pose: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.Pose();
                    if (object.yaw != null)
                        message.yaw = Number(object.yaw);
                    if (object.pitch != null)
                        message.pitch = Number(object.pitch);
                    if (object.roll != null)
                        message.roll = Number(object.roll);
                    if (object.deviceUs != null)
                        if ($util.Long)
                            message.deviceUs = $util.Long.fromValue(object.deviceUs, true);
                        else if (typeof object.deviceUs === "string")
                            message.deviceUs = parseInt(object.deviceUs, 10);
                        else if (typeof object.deviceUs === "number")
                            message.deviceUs = object.deviceUs;
                        else if (typeof object.deviceUs === "object")
                            message.deviceUs = new $util.LongBits(object.deviceUs.low >>> 0, object.deviceUs.high >>> 0).toNumber(true);
                    if (object.active != null)
                        message.active = Boolean(object.active);
                    if (object.source != null)
                        message.source = object.source >>> 0;
                    return message;
                };

                /**
                 * Creates a plain object from a Pose message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.Pose
                 * @static
                 * @param {eyepiece.rpc.v1.Pose} message Pose
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                Pose.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.defaults) {
                        object.yaw = 0;
                        object.pitch = 0;
                        object.roll = 0;
                        if ($util.Long) {
                            let long = new $util.Long(0, 0, true);
                            object.deviceUs = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : typeof BigInt !== "undefined" && options.longs === BigInt ? long.toBigInt() : long;
                        } else
                            object.deviceUs = options.longs === String ? "0" : typeof BigInt !== "undefined" && options.longs === BigInt ? BigInt("0") : 0;
                        object.active = false;
                        object.source = 0;
                    }
                    if (message.yaw != null && Object.hasOwnProperty.call(message, "yaw"))
                        object.yaw = options.json && !isFinite(message.yaw) ? String(message.yaw) : message.yaw;
                    if (message.pitch != null && Object.hasOwnProperty.call(message, "pitch"))
                        object.pitch = options.json && !isFinite(message.pitch) ? String(message.pitch) : message.pitch;
                    if (message.roll != null && Object.hasOwnProperty.call(message, "roll"))
                        object.roll = options.json && !isFinite(message.roll) ? String(message.roll) : message.roll;
                    if (message.deviceUs != null && Object.hasOwnProperty.call(message, "deviceUs"))
                        if (typeof BigInt !== "undefined" && options.longs === BigInt)
                            object.deviceUs = typeof message.deviceUs === "number" ? BigInt(message.deviceUs) : $util.Long.fromBits(message.deviceUs.low >>> 0, message.deviceUs.high >>> 0, true).toBigInt();
                        else if (typeof message.deviceUs === "number")
                            object.deviceUs = options.longs === String ? String(message.deviceUs) : message.deviceUs;
                        else
                            object.deviceUs = options.longs === String ? $util.Long.prototype.toString.call(message.deviceUs) : options.longs === Number ? new $util.LongBits(message.deviceUs.low >>> 0, message.deviceUs.high >>> 0).toNumber(true) : message.deviceUs;
                    if (message.active != null && Object.hasOwnProperty.call(message, "active"))
                        object.active = message.active;
                    if (message.source != null && Object.hasOwnProperty.call(message, "source"))
                        object.source = message.source;
                    return object;
                };

                /**
                 * Converts this Pose to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.Pose
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                Pose.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for Pose
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.Pose
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                Pose.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.Pose";
                };

                return Pose;
            })();

            v1.Metrics = (function() {

                /**
                 * Properties of a Metrics.
                 * @memberof eyepiece.rpc.v1
                 * @interface IMetrics
                 * @property {number|null} [intervalS] Metrics intervalS
                 * @property {number|null} [sentFps] Metrics sentFps
                 * @property {number|null} [mbps] Metrics mbps
                 * @property {number|null} [shownFps] Metrics shownFps
                 * @property {number|null} [latP50Ms] Metrics latP50Ms
                 * @property {number|null} [latP95Ms] Metrics latP95Ms
                 * @property {number|null} [capSendP50Ms] Metrics capSendP50Ms
                 * @property {number|null} [capSendP95Ms] Metrics capSendP95Ms
                 * @property {number|null} [capEncP50Ms] Metrics capEncP50Ms
                 * @property {number|null} [capEncP95Ms] Metrics capEncP95Ms
                 * @property {number|null} [encP50Ms] Metrics encP50Ms
                 * @property {number|null} [encP95Ms] Metrics encP95Ms
                 * @property {number|null} [decP50Ms] Metrics decP50Ms
                 * @property {number|null} [decP95Ms] Metrics decP95Ms
                 * @property {number|null} [cvtP50Ms] Metrics cvtP50Ms
                 * @property {number|null} [cvtP95Ms] Metrics cvtP95Ms
                 * @property {number|null} [recvP50Ms] Metrics recvP50Ms
                 * @property {number|null} [recvP95Ms] Metrics recvP95Ms
                 * @property {number|null} [qMax] Metrics qMax
                 * @property {number|null} [dropped] Metrics dropped
                 * @property {number|null} [skipped] Metrics skipped
                 * @property {number|null} [idr] Metrics idr
                 * @property {number|null} [capturedFps] Metrics capturedFps
                 * @property {number|null} [poseHz] Metrics poseHz
                 * @property {number|null} [rttMs] Metrics rttMs
                 * @property {number|null} [reportIntervalS] Metrics reportIntervalS
                 * @property {number|null} [sampledAtMs] Metrics sampledAtMs
                 */

                /**
                 * Constructs a new Metrics.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a Metrics.
                 * @implements IMetrics
                 * @constructor
                 * @param {eyepiece.rpc.v1.IMetrics=} [properties] Properties to set
                 */
                function Metrics(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Metrics intervalS.
                 * @member {number} intervalS
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.intervalS = 0;

                /**
                 * Metrics sentFps.
                 * @member {number} sentFps
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.sentFps = 0;

                /**
                 * Metrics mbps.
                 * @member {number} mbps
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.mbps = 0;

                /**
                 * Metrics shownFps.
                 * @member {number} shownFps
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.shownFps = 0;

                /**
                 * Metrics latP50Ms.
                 * @member {number} latP50Ms
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.latP50Ms = 0;

                /**
                 * Metrics latP95Ms.
                 * @member {number} latP95Ms
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.latP95Ms = 0;

                /**
                 * Metrics capSendP50Ms.
                 * @member {number} capSendP50Ms
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.capSendP50Ms = 0;

                /**
                 * Metrics capSendP95Ms.
                 * @member {number} capSendP95Ms
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.capSendP95Ms = 0;

                /**
                 * Metrics capEncP50Ms.
                 * @member {number} capEncP50Ms
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.capEncP50Ms = 0;

                /**
                 * Metrics capEncP95Ms.
                 * @member {number} capEncP95Ms
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.capEncP95Ms = 0;

                /**
                 * Metrics encP50Ms.
                 * @member {number} encP50Ms
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.encP50Ms = 0;

                /**
                 * Metrics encP95Ms.
                 * @member {number} encP95Ms
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.encP95Ms = 0;

                /**
                 * Metrics decP50Ms.
                 * @member {number} decP50Ms
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.decP50Ms = 0;

                /**
                 * Metrics decP95Ms.
                 * @member {number} decP95Ms
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.decP95Ms = 0;

                /**
                 * Metrics cvtP50Ms.
                 * @member {number} cvtP50Ms
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.cvtP50Ms = 0;

                /**
                 * Metrics cvtP95Ms.
                 * @member {number} cvtP95Ms
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.cvtP95Ms = 0;

                /**
                 * Metrics recvP50Ms.
                 * @member {number} recvP50Ms
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.recvP50Ms = 0;

                /**
                 * Metrics recvP95Ms.
                 * @member {number} recvP95Ms
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.recvP95Ms = 0;

                /**
                 * Metrics qMax.
                 * @member {number} qMax
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.qMax = 0;

                /**
                 * Metrics dropped.
                 * @member {number} dropped
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.dropped = 0;

                /**
                 * Metrics skipped.
                 * @member {number} skipped
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.skipped = 0;

                /**
                 * Metrics idr.
                 * @member {number} idr
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.idr = 0;

                /**
                 * Metrics capturedFps.
                 * @member {number} capturedFps
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.capturedFps = 0;

                /**
                 * Metrics poseHz.
                 * @member {number} poseHz
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.poseHz = 0;

                /**
                 * Metrics rttMs.
                 * @member {number} rttMs
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.rttMs = 0;

                /**
                 * Metrics reportIntervalS.
                 * @member {number} reportIntervalS
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.reportIntervalS = 0;

                /**
                 * Metrics sampledAtMs.
                 * @member {number} sampledAtMs
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 */
                Metrics.prototype.sampledAtMs = 0;

                /**
                 * Creates a new Metrics instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @static
                 * @param {eyepiece.rpc.v1.IMetrics=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.Metrics} Metrics instance
                 */
                Metrics.create = function create(properties) {
                    return new Metrics(properties);
                };

                /**
                 * Encodes the specified Metrics message. Does not implicitly {@link eyepiece.rpc.v1.Metrics.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @static
                 * @param {eyepiece.rpc.v1.IMetrics} message Metrics message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Metrics.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.intervalS != null && Object.hasOwnProperty.call(message, "intervalS"))
                        writer.uint32(/* id 1, wireType 1 =*/9).double(message.intervalS);
                    if (message.sentFps != null && Object.hasOwnProperty.call(message, "sentFps"))
                        writer.uint32(/* id 2, wireType 1 =*/17).double(message.sentFps);
                    if (message.mbps != null && Object.hasOwnProperty.call(message, "mbps"))
                        writer.uint32(/* id 3, wireType 1 =*/25).double(message.mbps);
                    if (message.shownFps != null && Object.hasOwnProperty.call(message, "shownFps"))
                        writer.uint32(/* id 4, wireType 1 =*/33).double(message.shownFps);
                    if (message.latP50Ms != null && Object.hasOwnProperty.call(message, "latP50Ms"))
                        writer.uint32(/* id 5, wireType 1 =*/41).double(message.latP50Ms);
                    if (message.latP95Ms != null && Object.hasOwnProperty.call(message, "latP95Ms"))
                        writer.uint32(/* id 6, wireType 1 =*/49).double(message.latP95Ms);
                    if (message.capSendP50Ms != null && Object.hasOwnProperty.call(message, "capSendP50Ms"))
                        writer.uint32(/* id 7, wireType 1 =*/57).double(message.capSendP50Ms);
                    if (message.capSendP95Ms != null && Object.hasOwnProperty.call(message, "capSendP95Ms"))
                        writer.uint32(/* id 8, wireType 1 =*/65).double(message.capSendP95Ms);
                    if (message.capEncP50Ms != null && Object.hasOwnProperty.call(message, "capEncP50Ms"))
                        writer.uint32(/* id 9, wireType 1 =*/73).double(message.capEncP50Ms);
                    if (message.capEncP95Ms != null && Object.hasOwnProperty.call(message, "capEncP95Ms"))
                        writer.uint32(/* id 10, wireType 1 =*/81).double(message.capEncP95Ms);
                    if (message.encP50Ms != null && Object.hasOwnProperty.call(message, "encP50Ms"))
                        writer.uint32(/* id 11, wireType 1 =*/89).double(message.encP50Ms);
                    if (message.encP95Ms != null && Object.hasOwnProperty.call(message, "encP95Ms"))
                        writer.uint32(/* id 12, wireType 1 =*/97).double(message.encP95Ms);
                    if (message.decP50Ms != null && Object.hasOwnProperty.call(message, "decP50Ms"))
                        writer.uint32(/* id 13, wireType 1 =*/105).double(message.decP50Ms);
                    if (message.decP95Ms != null && Object.hasOwnProperty.call(message, "decP95Ms"))
                        writer.uint32(/* id 14, wireType 1 =*/113).double(message.decP95Ms);
                    if (message.cvtP50Ms != null && Object.hasOwnProperty.call(message, "cvtP50Ms"))
                        writer.uint32(/* id 15, wireType 1 =*/121).double(message.cvtP50Ms);
                    if (message.cvtP95Ms != null && Object.hasOwnProperty.call(message, "cvtP95Ms"))
                        writer.uint32(/* id 16, wireType 1 =*/129).double(message.cvtP95Ms);
                    if (message.recvP50Ms != null && Object.hasOwnProperty.call(message, "recvP50Ms"))
                        writer.uint32(/* id 17, wireType 1 =*/137).double(message.recvP50Ms);
                    if (message.recvP95Ms != null && Object.hasOwnProperty.call(message, "recvP95Ms"))
                        writer.uint32(/* id 18, wireType 1 =*/145).double(message.recvP95Ms);
                    if (message.qMax != null && Object.hasOwnProperty.call(message, "qMax"))
                        writer.uint32(/* id 19, wireType 0 =*/152).uint32(message.qMax);
                    if (message.dropped != null && Object.hasOwnProperty.call(message, "dropped"))
                        writer.uint32(/* id 20, wireType 0 =*/160).uint32(message.dropped);
                    if (message.skipped != null && Object.hasOwnProperty.call(message, "skipped"))
                        writer.uint32(/* id 21, wireType 0 =*/168).uint32(message.skipped);
                    if (message.idr != null && Object.hasOwnProperty.call(message, "idr"))
                        writer.uint32(/* id 22, wireType 0 =*/176).uint32(message.idr);
                    if (message.capturedFps != null && Object.hasOwnProperty.call(message, "capturedFps"))
                        writer.uint32(/* id 23, wireType 1 =*/185).double(message.capturedFps);
                    if (message.poseHz != null && Object.hasOwnProperty.call(message, "poseHz"))
                        writer.uint32(/* id 24, wireType 1 =*/193).double(message.poseHz);
                    if (message.rttMs != null && Object.hasOwnProperty.call(message, "rttMs"))
                        writer.uint32(/* id 25, wireType 1 =*/201).double(message.rttMs);
                    if (message.reportIntervalS != null && Object.hasOwnProperty.call(message, "reportIntervalS"))
                        writer.uint32(/* id 26, wireType 1 =*/209).double(message.reportIntervalS);
                    if (message.sampledAtMs != null && Object.hasOwnProperty.call(message, "sampledAtMs"))
                        writer.uint32(/* id 27, wireType 1 =*/217).double(message.sampledAtMs);
                    return writer;
                };

                /**
                 * Encodes the specified Metrics message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Metrics.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @static
                 * @param {eyepiece.rpc.v1.IMetrics} message Metrics message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Metrics.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a Metrics message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.Metrics} Metrics
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Metrics.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.Metrics();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                message.intervalS = reader.double();
                                break;
                            }
                        case 2: {
                                message.sentFps = reader.double();
                                break;
                            }
                        case 3: {
                                message.mbps = reader.double();
                                break;
                            }
                        case 4: {
                                message.shownFps = reader.double();
                                break;
                            }
                        case 5: {
                                message.latP50Ms = reader.double();
                                break;
                            }
                        case 6: {
                                message.latP95Ms = reader.double();
                                break;
                            }
                        case 7: {
                                message.capSendP50Ms = reader.double();
                                break;
                            }
                        case 8: {
                                message.capSendP95Ms = reader.double();
                                break;
                            }
                        case 9: {
                                message.capEncP50Ms = reader.double();
                                break;
                            }
                        case 10: {
                                message.capEncP95Ms = reader.double();
                                break;
                            }
                        case 11: {
                                message.encP50Ms = reader.double();
                                break;
                            }
                        case 12: {
                                message.encP95Ms = reader.double();
                                break;
                            }
                        case 13: {
                                message.decP50Ms = reader.double();
                                break;
                            }
                        case 14: {
                                message.decP95Ms = reader.double();
                                break;
                            }
                        case 15: {
                                message.cvtP50Ms = reader.double();
                                break;
                            }
                        case 16: {
                                message.cvtP95Ms = reader.double();
                                break;
                            }
                        case 17: {
                                message.recvP50Ms = reader.double();
                                break;
                            }
                        case 18: {
                                message.recvP95Ms = reader.double();
                                break;
                            }
                        case 19: {
                                message.qMax = reader.uint32();
                                break;
                            }
                        case 20: {
                                message.dropped = reader.uint32();
                                break;
                            }
                        case 21: {
                                message.skipped = reader.uint32();
                                break;
                            }
                        case 22: {
                                message.idr = reader.uint32();
                                break;
                            }
                        case 23: {
                                message.capturedFps = reader.double();
                                break;
                            }
                        case 24: {
                                message.poseHz = reader.double();
                                break;
                            }
                        case 25: {
                                message.rttMs = reader.double();
                                break;
                            }
                        case 26: {
                                message.reportIntervalS = reader.double();
                                break;
                            }
                        case 27: {
                                message.sampledAtMs = reader.double();
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a Metrics message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.Metrics} Metrics
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Metrics.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a Metrics message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                Metrics.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.intervalS != null && Object.hasOwnProperty.call(message, "intervalS"))
                        if (typeof message.intervalS !== "number")
                            return "intervalS: number expected";
                    if (message.sentFps != null && Object.hasOwnProperty.call(message, "sentFps"))
                        if (typeof message.sentFps !== "number")
                            return "sentFps: number expected";
                    if (message.mbps != null && Object.hasOwnProperty.call(message, "mbps"))
                        if (typeof message.mbps !== "number")
                            return "mbps: number expected";
                    if (message.shownFps != null && Object.hasOwnProperty.call(message, "shownFps"))
                        if (typeof message.shownFps !== "number")
                            return "shownFps: number expected";
                    if (message.latP50Ms != null && Object.hasOwnProperty.call(message, "latP50Ms"))
                        if (typeof message.latP50Ms !== "number")
                            return "latP50Ms: number expected";
                    if (message.latP95Ms != null && Object.hasOwnProperty.call(message, "latP95Ms"))
                        if (typeof message.latP95Ms !== "number")
                            return "latP95Ms: number expected";
                    if (message.capSendP50Ms != null && Object.hasOwnProperty.call(message, "capSendP50Ms"))
                        if (typeof message.capSendP50Ms !== "number")
                            return "capSendP50Ms: number expected";
                    if (message.capSendP95Ms != null && Object.hasOwnProperty.call(message, "capSendP95Ms"))
                        if (typeof message.capSendP95Ms !== "number")
                            return "capSendP95Ms: number expected";
                    if (message.capEncP50Ms != null && Object.hasOwnProperty.call(message, "capEncP50Ms"))
                        if (typeof message.capEncP50Ms !== "number")
                            return "capEncP50Ms: number expected";
                    if (message.capEncP95Ms != null && Object.hasOwnProperty.call(message, "capEncP95Ms"))
                        if (typeof message.capEncP95Ms !== "number")
                            return "capEncP95Ms: number expected";
                    if (message.encP50Ms != null && Object.hasOwnProperty.call(message, "encP50Ms"))
                        if (typeof message.encP50Ms !== "number")
                            return "encP50Ms: number expected";
                    if (message.encP95Ms != null && Object.hasOwnProperty.call(message, "encP95Ms"))
                        if (typeof message.encP95Ms !== "number")
                            return "encP95Ms: number expected";
                    if (message.decP50Ms != null && Object.hasOwnProperty.call(message, "decP50Ms"))
                        if (typeof message.decP50Ms !== "number")
                            return "decP50Ms: number expected";
                    if (message.decP95Ms != null && Object.hasOwnProperty.call(message, "decP95Ms"))
                        if (typeof message.decP95Ms !== "number")
                            return "decP95Ms: number expected";
                    if (message.cvtP50Ms != null && Object.hasOwnProperty.call(message, "cvtP50Ms"))
                        if (typeof message.cvtP50Ms !== "number")
                            return "cvtP50Ms: number expected";
                    if (message.cvtP95Ms != null && Object.hasOwnProperty.call(message, "cvtP95Ms"))
                        if (typeof message.cvtP95Ms !== "number")
                            return "cvtP95Ms: number expected";
                    if (message.recvP50Ms != null && Object.hasOwnProperty.call(message, "recvP50Ms"))
                        if (typeof message.recvP50Ms !== "number")
                            return "recvP50Ms: number expected";
                    if (message.recvP95Ms != null && Object.hasOwnProperty.call(message, "recvP95Ms"))
                        if (typeof message.recvP95Ms !== "number")
                            return "recvP95Ms: number expected";
                    if (message.qMax != null && Object.hasOwnProperty.call(message, "qMax"))
                        if (!$util.isInteger(message.qMax))
                            return "qMax: integer expected";
                    if (message.dropped != null && Object.hasOwnProperty.call(message, "dropped"))
                        if (!$util.isInteger(message.dropped))
                            return "dropped: integer expected";
                    if (message.skipped != null && Object.hasOwnProperty.call(message, "skipped"))
                        if (!$util.isInteger(message.skipped))
                            return "skipped: integer expected";
                    if (message.idr != null && Object.hasOwnProperty.call(message, "idr"))
                        if (!$util.isInteger(message.idr))
                            return "idr: integer expected";
                    if (message.capturedFps != null && Object.hasOwnProperty.call(message, "capturedFps"))
                        if (typeof message.capturedFps !== "number")
                            return "capturedFps: number expected";
                    if (message.poseHz != null && Object.hasOwnProperty.call(message, "poseHz"))
                        if (typeof message.poseHz !== "number")
                            return "poseHz: number expected";
                    if (message.rttMs != null && Object.hasOwnProperty.call(message, "rttMs"))
                        if (typeof message.rttMs !== "number")
                            return "rttMs: number expected";
                    if (message.reportIntervalS != null && Object.hasOwnProperty.call(message, "reportIntervalS"))
                        if (typeof message.reportIntervalS !== "number")
                            return "reportIntervalS: number expected";
                    if (message.sampledAtMs != null && Object.hasOwnProperty.call(message, "sampledAtMs"))
                        if (typeof message.sampledAtMs !== "number")
                            return "sampledAtMs: number expected";
                    return null;
                };

                /**
                 * Creates a Metrics message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.Metrics} Metrics
                 */
                Metrics.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.Metrics)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.Metrics: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.Metrics();
                    if (object.intervalS != null)
                        message.intervalS = Number(object.intervalS);
                    if (object.sentFps != null)
                        message.sentFps = Number(object.sentFps);
                    if (object.mbps != null)
                        message.mbps = Number(object.mbps);
                    if (object.shownFps != null)
                        message.shownFps = Number(object.shownFps);
                    if (object.latP50Ms != null)
                        message.latP50Ms = Number(object.latP50Ms);
                    if (object.latP95Ms != null)
                        message.latP95Ms = Number(object.latP95Ms);
                    if (object.capSendP50Ms != null)
                        message.capSendP50Ms = Number(object.capSendP50Ms);
                    if (object.capSendP95Ms != null)
                        message.capSendP95Ms = Number(object.capSendP95Ms);
                    if (object.capEncP50Ms != null)
                        message.capEncP50Ms = Number(object.capEncP50Ms);
                    if (object.capEncP95Ms != null)
                        message.capEncP95Ms = Number(object.capEncP95Ms);
                    if (object.encP50Ms != null)
                        message.encP50Ms = Number(object.encP50Ms);
                    if (object.encP95Ms != null)
                        message.encP95Ms = Number(object.encP95Ms);
                    if (object.decP50Ms != null)
                        message.decP50Ms = Number(object.decP50Ms);
                    if (object.decP95Ms != null)
                        message.decP95Ms = Number(object.decP95Ms);
                    if (object.cvtP50Ms != null)
                        message.cvtP50Ms = Number(object.cvtP50Ms);
                    if (object.cvtP95Ms != null)
                        message.cvtP95Ms = Number(object.cvtP95Ms);
                    if (object.recvP50Ms != null)
                        message.recvP50Ms = Number(object.recvP50Ms);
                    if (object.recvP95Ms != null)
                        message.recvP95Ms = Number(object.recvP95Ms);
                    if (object.qMax != null)
                        message.qMax = object.qMax >>> 0;
                    if (object.dropped != null)
                        message.dropped = object.dropped >>> 0;
                    if (object.skipped != null)
                        message.skipped = object.skipped >>> 0;
                    if (object.idr != null)
                        message.idr = object.idr >>> 0;
                    if (object.capturedFps != null)
                        message.capturedFps = Number(object.capturedFps);
                    if (object.poseHz != null)
                        message.poseHz = Number(object.poseHz);
                    if (object.rttMs != null)
                        message.rttMs = Number(object.rttMs);
                    if (object.reportIntervalS != null)
                        message.reportIntervalS = Number(object.reportIntervalS);
                    if (object.sampledAtMs != null)
                        message.sampledAtMs = Number(object.sampledAtMs);
                    return message;
                };

                /**
                 * Creates a plain object from a Metrics message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @static
                 * @param {eyepiece.rpc.v1.Metrics} message Metrics
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                Metrics.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.defaults) {
                        object.intervalS = 0;
                        object.sentFps = 0;
                        object.mbps = 0;
                        object.shownFps = 0;
                        object.latP50Ms = 0;
                        object.latP95Ms = 0;
                        object.capSendP50Ms = 0;
                        object.capSendP95Ms = 0;
                        object.capEncP50Ms = 0;
                        object.capEncP95Ms = 0;
                        object.encP50Ms = 0;
                        object.encP95Ms = 0;
                        object.decP50Ms = 0;
                        object.decP95Ms = 0;
                        object.cvtP50Ms = 0;
                        object.cvtP95Ms = 0;
                        object.recvP50Ms = 0;
                        object.recvP95Ms = 0;
                        object.qMax = 0;
                        object.dropped = 0;
                        object.skipped = 0;
                        object.idr = 0;
                        object.capturedFps = 0;
                        object.poseHz = 0;
                        object.rttMs = 0;
                        object.reportIntervalS = 0;
                        object.sampledAtMs = 0;
                    }
                    if (message.intervalS != null && Object.hasOwnProperty.call(message, "intervalS"))
                        object.intervalS = options.json && !isFinite(message.intervalS) ? String(message.intervalS) : message.intervalS;
                    if (message.sentFps != null && Object.hasOwnProperty.call(message, "sentFps"))
                        object.sentFps = options.json && !isFinite(message.sentFps) ? String(message.sentFps) : message.sentFps;
                    if (message.mbps != null && Object.hasOwnProperty.call(message, "mbps"))
                        object.mbps = options.json && !isFinite(message.mbps) ? String(message.mbps) : message.mbps;
                    if (message.shownFps != null && Object.hasOwnProperty.call(message, "shownFps"))
                        object.shownFps = options.json && !isFinite(message.shownFps) ? String(message.shownFps) : message.shownFps;
                    if (message.latP50Ms != null && Object.hasOwnProperty.call(message, "latP50Ms"))
                        object.latP50Ms = options.json && !isFinite(message.latP50Ms) ? String(message.latP50Ms) : message.latP50Ms;
                    if (message.latP95Ms != null && Object.hasOwnProperty.call(message, "latP95Ms"))
                        object.latP95Ms = options.json && !isFinite(message.latP95Ms) ? String(message.latP95Ms) : message.latP95Ms;
                    if (message.capSendP50Ms != null && Object.hasOwnProperty.call(message, "capSendP50Ms"))
                        object.capSendP50Ms = options.json && !isFinite(message.capSendP50Ms) ? String(message.capSendP50Ms) : message.capSendP50Ms;
                    if (message.capSendP95Ms != null && Object.hasOwnProperty.call(message, "capSendP95Ms"))
                        object.capSendP95Ms = options.json && !isFinite(message.capSendP95Ms) ? String(message.capSendP95Ms) : message.capSendP95Ms;
                    if (message.capEncP50Ms != null && Object.hasOwnProperty.call(message, "capEncP50Ms"))
                        object.capEncP50Ms = options.json && !isFinite(message.capEncP50Ms) ? String(message.capEncP50Ms) : message.capEncP50Ms;
                    if (message.capEncP95Ms != null && Object.hasOwnProperty.call(message, "capEncP95Ms"))
                        object.capEncP95Ms = options.json && !isFinite(message.capEncP95Ms) ? String(message.capEncP95Ms) : message.capEncP95Ms;
                    if (message.encP50Ms != null && Object.hasOwnProperty.call(message, "encP50Ms"))
                        object.encP50Ms = options.json && !isFinite(message.encP50Ms) ? String(message.encP50Ms) : message.encP50Ms;
                    if (message.encP95Ms != null && Object.hasOwnProperty.call(message, "encP95Ms"))
                        object.encP95Ms = options.json && !isFinite(message.encP95Ms) ? String(message.encP95Ms) : message.encP95Ms;
                    if (message.decP50Ms != null && Object.hasOwnProperty.call(message, "decP50Ms"))
                        object.decP50Ms = options.json && !isFinite(message.decP50Ms) ? String(message.decP50Ms) : message.decP50Ms;
                    if (message.decP95Ms != null && Object.hasOwnProperty.call(message, "decP95Ms"))
                        object.decP95Ms = options.json && !isFinite(message.decP95Ms) ? String(message.decP95Ms) : message.decP95Ms;
                    if (message.cvtP50Ms != null && Object.hasOwnProperty.call(message, "cvtP50Ms"))
                        object.cvtP50Ms = options.json && !isFinite(message.cvtP50Ms) ? String(message.cvtP50Ms) : message.cvtP50Ms;
                    if (message.cvtP95Ms != null && Object.hasOwnProperty.call(message, "cvtP95Ms"))
                        object.cvtP95Ms = options.json && !isFinite(message.cvtP95Ms) ? String(message.cvtP95Ms) : message.cvtP95Ms;
                    if (message.recvP50Ms != null && Object.hasOwnProperty.call(message, "recvP50Ms"))
                        object.recvP50Ms = options.json && !isFinite(message.recvP50Ms) ? String(message.recvP50Ms) : message.recvP50Ms;
                    if (message.recvP95Ms != null && Object.hasOwnProperty.call(message, "recvP95Ms"))
                        object.recvP95Ms = options.json && !isFinite(message.recvP95Ms) ? String(message.recvP95Ms) : message.recvP95Ms;
                    if (message.qMax != null && Object.hasOwnProperty.call(message, "qMax"))
                        object.qMax = message.qMax;
                    if (message.dropped != null && Object.hasOwnProperty.call(message, "dropped"))
                        object.dropped = message.dropped;
                    if (message.skipped != null && Object.hasOwnProperty.call(message, "skipped"))
                        object.skipped = message.skipped;
                    if (message.idr != null && Object.hasOwnProperty.call(message, "idr"))
                        object.idr = message.idr;
                    if (message.capturedFps != null && Object.hasOwnProperty.call(message, "capturedFps"))
                        object.capturedFps = options.json && !isFinite(message.capturedFps) ? String(message.capturedFps) : message.capturedFps;
                    if (message.poseHz != null && Object.hasOwnProperty.call(message, "poseHz"))
                        object.poseHz = options.json && !isFinite(message.poseHz) ? String(message.poseHz) : message.poseHz;
                    if (message.rttMs != null && Object.hasOwnProperty.call(message, "rttMs"))
                        object.rttMs = options.json && !isFinite(message.rttMs) ? String(message.rttMs) : message.rttMs;
                    if (message.reportIntervalS != null && Object.hasOwnProperty.call(message, "reportIntervalS"))
                        object.reportIntervalS = options.json && !isFinite(message.reportIntervalS) ? String(message.reportIntervalS) : message.reportIntervalS;
                    if (message.sampledAtMs != null && Object.hasOwnProperty.call(message, "sampledAtMs"))
                        object.sampledAtMs = options.json && !isFinite(message.sampledAtMs) ? String(message.sampledAtMs) : message.sampledAtMs;
                    return object;
                };

                /**
                 * Converts this Metrics to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                Metrics.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for Metrics
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.Metrics
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                Metrics.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.Metrics";
                };

                return Metrics;
            })();

            v1.Preview = (function() {

                /**
                 * Properties of a Preview.
                 * @memberof eyepiece.rpc.v1
                 * @interface IPreview
                 * @property {Uint8Array|null} [accessUnit] Preview accessUnit
                 * @property {boolean|null} [key] Preview key
                 * @property {string|null} [codec] Preview codec
                 * @property {number|Long|null} [timestampUs] Preview timestampUs
                 * @property {boolean|null} [discontinuity] Preview discontinuity
                 * @property {boolean|null} [packedAlpha] Preview packedAlpha
                 */

                /**
                 * Constructs a new Preview.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a Preview.
                 * @implements IPreview
                 * @constructor
                 * @param {eyepiece.rpc.v1.IPreview=} [properties] Properties to set
                 */
                function Preview(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Preview accessUnit.
                 * @member {Uint8Array} accessUnit
                 * @memberof eyepiece.rpc.v1.Preview
                 * @instance
                 */
                Preview.prototype.accessUnit = $util.newBuffer([]);

                /**
                 * Preview key.
                 * @member {boolean} key
                 * @memberof eyepiece.rpc.v1.Preview
                 * @instance
                 */
                Preview.prototype.key = false;

                /**
                 * Preview codec.
                 * @member {string} codec
                 * @memberof eyepiece.rpc.v1.Preview
                 * @instance
                 */
                Preview.prototype.codec = "";

                /**
                 * Preview timestampUs.
                 * @member {number|Long} timestampUs
                 * @memberof eyepiece.rpc.v1.Preview
                 * @instance
                 */
                Preview.prototype.timestampUs = $util.Long ? $util.Long.fromBits(0,0,true) : 0;

                /**
                 * Preview discontinuity.
                 * @member {boolean} discontinuity
                 * @memberof eyepiece.rpc.v1.Preview
                 * @instance
                 */
                Preview.prototype.discontinuity = false;

                /**
                 * Preview packedAlpha.
                 * @member {boolean} packedAlpha
                 * @memberof eyepiece.rpc.v1.Preview
                 * @instance
                 */
                Preview.prototype.packedAlpha = false;

                /**
                 * Creates a new Preview instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.Preview
                 * @static
                 * @param {eyepiece.rpc.v1.IPreview=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.Preview} Preview instance
                 */
                Preview.create = function create(properties) {
                    return new Preview(properties);
                };

                /**
                 * Encodes the specified Preview message. Does not implicitly {@link eyepiece.rpc.v1.Preview.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.Preview
                 * @static
                 * @param {eyepiece.rpc.v1.IPreview} message Preview message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Preview.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.accessUnit != null && Object.hasOwnProperty.call(message, "accessUnit"))
                        writer.uint32(/* id 1, wireType 2 =*/10).bytes(message.accessUnit);
                    if (message.key != null && Object.hasOwnProperty.call(message, "key"))
                        writer.uint32(/* id 2, wireType 0 =*/16).bool(message.key);
                    if (message.codec != null && Object.hasOwnProperty.call(message, "codec"))
                        writer.uint32(/* id 3, wireType 2 =*/26).string(message.codec);
                    if (message.timestampUs != null && Object.hasOwnProperty.call(message, "timestampUs"))
                        writer.uint32(/* id 4, wireType 0 =*/32).uint64(message.timestampUs);
                    if (message.discontinuity != null && Object.hasOwnProperty.call(message, "discontinuity"))
                        writer.uint32(/* id 5, wireType 0 =*/40).bool(message.discontinuity);
                    if (message.packedAlpha != null && Object.hasOwnProperty.call(message, "packedAlpha"))
                        writer.uint32(/* id 6, wireType 0 =*/48).bool(message.packedAlpha);
                    return writer;
                };

                /**
                 * Encodes the specified Preview message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Preview.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.Preview
                 * @static
                 * @param {eyepiece.rpc.v1.IPreview} message Preview message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Preview.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a Preview message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.Preview
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.Preview} Preview
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Preview.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.Preview();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                message.accessUnit = reader.bytes();
                                break;
                            }
                        case 2: {
                                message.key = reader.bool();
                                break;
                            }
                        case 3: {
                                message.codec = reader.string();
                                break;
                            }
                        case 4: {
                                message.timestampUs = reader.uint64();
                                break;
                            }
                        case 5: {
                                message.discontinuity = reader.bool();
                                break;
                            }
                        case 6: {
                                message.packedAlpha = reader.bool();
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a Preview message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.Preview
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.Preview} Preview
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Preview.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a Preview message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.Preview
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                Preview.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.accessUnit != null && Object.hasOwnProperty.call(message, "accessUnit"))
                        if (!(message.accessUnit && typeof message.accessUnit.length === "number" || $util.isString(message.accessUnit)))
                            return "accessUnit: buffer expected";
                    if (message.key != null && Object.hasOwnProperty.call(message, "key"))
                        if (typeof message.key !== "boolean")
                            return "key: boolean expected";
                    if (message.codec != null && Object.hasOwnProperty.call(message, "codec"))
                        if (!$util.isString(message.codec))
                            return "codec: string expected";
                    if (message.timestampUs != null && Object.hasOwnProperty.call(message, "timestampUs"))
                        if (!$util.isInteger(message.timestampUs) && !(message.timestampUs && $util.isInteger(message.timestampUs.low) && $util.isInteger(message.timestampUs.high)))
                            return "timestampUs: integer|Long expected";
                    if (message.discontinuity != null && Object.hasOwnProperty.call(message, "discontinuity"))
                        if (typeof message.discontinuity !== "boolean")
                            return "discontinuity: boolean expected";
                    if (message.packedAlpha != null && Object.hasOwnProperty.call(message, "packedAlpha"))
                        if (typeof message.packedAlpha !== "boolean")
                            return "packedAlpha: boolean expected";
                    return null;
                };

                /**
                 * Creates a Preview message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.Preview
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.Preview} Preview
                 */
                Preview.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.Preview)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.Preview: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.Preview();
                    if (object.accessUnit != null)
                        if (typeof object.accessUnit === "string")
                            $util.base64.decode(object.accessUnit, message.accessUnit = $util.newBuffer($util.base64.length(object.accessUnit)), 0);
                        else if (object.accessUnit.length >= 0)
                            message.accessUnit = object.accessUnit;
                    if (object.key != null)
                        message.key = Boolean(object.key);
                    if (object.codec != null)
                        message.codec = String(object.codec);
                    if (object.timestampUs != null)
                        if ($util.Long)
                            message.timestampUs = $util.Long.fromValue(object.timestampUs, true);
                        else if (typeof object.timestampUs === "string")
                            message.timestampUs = parseInt(object.timestampUs, 10);
                        else if (typeof object.timestampUs === "number")
                            message.timestampUs = object.timestampUs;
                        else if (typeof object.timestampUs === "object")
                            message.timestampUs = new $util.LongBits(object.timestampUs.low >>> 0, object.timestampUs.high >>> 0).toNumber(true);
                    if (object.discontinuity != null)
                        message.discontinuity = Boolean(object.discontinuity);
                    if (object.packedAlpha != null)
                        message.packedAlpha = Boolean(object.packedAlpha);
                    return message;
                };

                /**
                 * Creates a plain object from a Preview message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.Preview
                 * @static
                 * @param {eyepiece.rpc.v1.Preview} message Preview
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                Preview.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.defaults) {
                        if (options.bytes === String)
                            object.accessUnit = "";
                        else {
                            object.accessUnit = [];
                            if (options.bytes !== Array)
                                object.accessUnit = $util.newBuffer(object.accessUnit);
                        }
                        object.key = false;
                        object.codec = "";
                        if ($util.Long) {
                            let long = new $util.Long(0, 0, true);
                            object.timestampUs = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : typeof BigInt !== "undefined" && options.longs === BigInt ? long.toBigInt() : long;
                        } else
                            object.timestampUs = options.longs === String ? "0" : typeof BigInt !== "undefined" && options.longs === BigInt ? BigInt("0") : 0;
                        object.discontinuity = false;
                        object.packedAlpha = false;
                    }
                    if (message.accessUnit != null && Object.hasOwnProperty.call(message, "accessUnit"))
                        object.accessUnit = options.bytes === String ? $util.base64.encode(message.accessUnit, 0, message.accessUnit.length) : options.bytes === Array ? Array.prototype.slice.call(message.accessUnit) : message.accessUnit;
                    if (message.key != null && Object.hasOwnProperty.call(message, "key"))
                        object.key = message.key;
                    if (message.codec != null && Object.hasOwnProperty.call(message, "codec"))
                        object.codec = message.codec;
                    if (message.timestampUs != null && Object.hasOwnProperty.call(message, "timestampUs"))
                        if (typeof BigInt !== "undefined" && options.longs === BigInt)
                            object.timestampUs = typeof message.timestampUs === "number" ? BigInt(message.timestampUs) : $util.Long.fromBits(message.timestampUs.low >>> 0, message.timestampUs.high >>> 0, true).toBigInt();
                        else if (typeof message.timestampUs === "number")
                            object.timestampUs = options.longs === String ? String(message.timestampUs) : message.timestampUs;
                        else
                            object.timestampUs = options.longs === String ? $util.Long.prototype.toString.call(message.timestampUs) : options.longs === Number ? new $util.LongBits(message.timestampUs.low >>> 0, message.timestampUs.high >>> 0).toNumber(true) : message.timestampUs;
                    if (message.discontinuity != null && Object.hasOwnProperty.call(message, "discontinuity"))
                        object.discontinuity = message.discontinuity;
                    if (message.packedAlpha != null && Object.hasOwnProperty.call(message, "packedAlpha"))
                        object.packedAlpha = message.packedAlpha;
                    return object;
                };

                /**
                 * Converts this Preview to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.Preview
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                Preview.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for Preview
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.Preview
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                Preview.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.Preview";
                };

                return Preview;
            })();

            v1.KmsOutputs = (function() {

                /**
                 * Properties of a KmsOutputs.
                 * @memberof eyepiece.rpc.v1
                 * @interface IKmsOutputs
                 * @property {Array.<string>|null} [outputs] KmsOutputs outputs
                 */

                /**
                 * Constructs a new KmsOutputs.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a KmsOutputs.
                 * @implements IKmsOutputs
                 * @constructor
                 * @param {eyepiece.rpc.v1.IKmsOutputs=} [properties] Properties to set
                 */
                function KmsOutputs(properties) {
                    this.outputs = [];
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * KmsOutputs outputs.
                 * @member {Array.<string>} outputs
                 * @memberof eyepiece.rpc.v1.KmsOutputs
                 * @instance
                 */
                KmsOutputs.prototype.outputs = $util.emptyArray;

                /**
                 * Creates a new KmsOutputs instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.KmsOutputs
                 * @static
                 * @param {eyepiece.rpc.v1.IKmsOutputs=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.KmsOutputs} KmsOutputs instance
                 */
                KmsOutputs.create = function create(properties) {
                    return new KmsOutputs(properties);
                };

                /**
                 * Encodes the specified KmsOutputs message. Does not implicitly {@link eyepiece.rpc.v1.KmsOutputs.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.KmsOutputs
                 * @static
                 * @param {eyepiece.rpc.v1.IKmsOutputs} message KmsOutputs message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                KmsOutputs.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.outputs != null && message.outputs.length)
                        for (let i = 0; i < message.outputs.length; ++i)
                            writer.uint32(/* id 1, wireType 2 =*/10).string(message.outputs[i]);
                    return writer;
                };

                /**
                 * Encodes the specified KmsOutputs message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.KmsOutputs.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.KmsOutputs
                 * @static
                 * @param {eyepiece.rpc.v1.IKmsOutputs} message KmsOutputs message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                KmsOutputs.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a KmsOutputs message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.KmsOutputs
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.KmsOutputs} KmsOutputs
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                KmsOutputs.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.KmsOutputs();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                if (!(message.outputs && message.outputs.length))
                                    message.outputs = [];
                                message.outputs.push(reader.string());
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a KmsOutputs message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.KmsOutputs
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.KmsOutputs} KmsOutputs
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                KmsOutputs.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a KmsOutputs message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.KmsOutputs
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                KmsOutputs.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.outputs != null && Object.hasOwnProperty.call(message, "outputs")) {
                        if (!Array.isArray(message.outputs))
                            return "outputs: array expected";
                        for (let i = 0; i < message.outputs.length; ++i)
                            if (!$util.isString(message.outputs[i]))
                                return "outputs: string[] expected";
                    }
                    return null;
                };

                /**
                 * Creates a KmsOutputs message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.KmsOutputs
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.KmsOutputs} KmsOutputs
                 */
                KmsOutputs.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.KmsOutputs)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.KmsOutputs: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.KmsOutputs();
                    if (object.outputs) {
                        if (!Array.isArray(object.outputs))
                            throw TypeError(".eyepiece.rpc.v1.KmsOutputs.outputs: array expected");
                        message.outputs = [];
                        for (let i = 0; i < object.outputs.length; ++i)
                            message.outputs[i] = String(object.outputs[i]);
                    }
                    return message;
                };

                /**
                 * Creates a plain object from a KmsOutputs message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.KmsOutputs
                 * @static
                 * @param {eyepiece.rpc.v1.KmsOutputs} message KmsOutputs
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                KmsOutputs.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.arrays || options.defaults)
                        object.outputs = [];
                    if (message.outputs && message.outputs.length) {
                        object.outputs = [];
                        for (let j = 0; j < message.outputs.length; ++j)
                            object.outputs[j] = message.outputs[j];
                    }
                    return object;
                };

                /**
                 * Converts this KmsOutputs to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.KmsOutputs
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                KmsOutputs.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for KmsOutputs
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.KmsOutputs
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                KmsOutputs.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.KmsOutputs";
                };

                return KmsOutputs;
            })();

            v1.Video = (function() {

                /**
                 * Properties of a Video.
                 * @memberof eyepiece.rpc.v1
                 * @interface IVideo
                 * @property {number|Long|null} [ptsUs] Video ptsUs
                 * @property {number|null} [codec] Video codec
                 * @property {boolean|null} [key] Video key
                 * @property {Uint8Array|null} [accessUnit] Video accessUnit
                 */

                /**
                 * Constructs a new Video.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a Video.
                 * @implements IVideo
                 * @constructor
                 * @param {eyepiece.rpc.v1.IVideo=} [properties] Properties to set
                 */
                function Video(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Video ptsUs.
                 * @member {number|Long} ptsUs
                 * @memberof eyepiece.rpc.v1.Video
                 * @instance
                 */
                Video.prototype.ptsUs = $util.Long ? $util.Long.fromBits(0,0,true) : 0;

                /**
                 * Video codec.
                 * @member {number} codec
                 * @memberof eyepiece.rpc.v1.Video
                 * @instance
                 */
                Video.prototype.codec = 0;

                /**
                 * Video key.
                 * @member {boolean} key
                 * @memberof eyepiece.rpc.v1.Video
                 * @instance
                 */
                Video.prototype.key = false;

                /**
                 * Video accessUnit.
                 * @member {Uint8Array} accessUnit
                 * @memberof eyepiece.rpc.v1.Video
                 * @instance
                 */
                Video.prototype.accessUnit = $util.newBuffer([]);

                /**
                 * Creates a new Video instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.Video
                 * @static
                 * @param {eyepiece.rpc.v1.IVideo=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.Video} Video instance
                 */
                Video.create = function create(properties) {
                    return new Video(properties);
                };

                /**
                 * Encodes the specified Video message. Does not implicitly {@link eyepiece.rpc.v1.Video.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.Video
                 * @static
                 * @param {eyepiece.rpc.v1.IVideo} message Video message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Video.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.ptsUs != null && Object.hasOwnProperty.call(message, "ptsUs"))
                        writer.uint32(/* id 1, wireType 0 =*/8).uint64(message.ptsUs);
                    if (message.codec != null && Object.hasOwnProperty.call(message, "codec"))
                        writer.uint32(/* id 2, wireType 0 =*/16).uint32(message.codec);
                    if (message.key != null && Object.hasOwnProperty.call(message, "key"))
                        writer.uint32(/* id 3, wireType 0 =*/24).bool(message.key);
                    if (message.accessUnit != null && Object.hasOwnProperty.call(message, "accessUnit"))
                        writer.uint32(/* id 4, wireType 2 =*/34).bytes(message.accessUnit);
                    return writer;
                };

                /**
                 * Encodes the specified Video message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Video.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.Video
                 * @static
                 * @param {eyepiece.rpc.v1.IVideo} message Video message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Video.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a Video message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.Video
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.Video} Video
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Video.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.Video();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                message.ptsUs = reader.uint64();
                                break;
                            }
                        case 2: {
                                message.codec = reader.uint32();
                                break;
                            }
                        case 3: {
                                message.key = reader.bool();
                                break;
                            }
                        case 4: {
                                message.accessUnit = reader.bytes();
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a Video message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.Video
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.Video} Video
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Video.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a Video message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.Video
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                Video.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.ptsUs != null && Object.hasOwnProperty.call(message, "ptsUs"))
                        if (!$util.isInteger(message.ptsUs) && !(message.ptsUs && $util.isInteger(message.ptsUs.low) && $util.isInteger(message.ptsUs.high)))
                            return "ptsUs: integer|Long expected";
                    if (message.codec != null && Object.hasOwnProperty.call(message, "codec"))
                        if (!$util.isInteger(message.codec))
                            return "codec: integer expected";
                    if (message.key != null && Object.hasOwnProperty.call(message, "key"))
                        if (typeof message.key !== "boolean")
                            return "key: boolean expected";
                    if (message.accessUnit != null && Object.hasOwnProperty.call(message, "accessUnit"))
                        if (!(message.accessUnit && typeof message.accessUnit.length === "number" || $util.isString(message.accessUnit)))
                            return "accessUnit: buffer expected";
                    return null;
                };

                /**
                 * Creates a Video message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.Video
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.Video} Video
                 */
                Video.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.Video)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.Video: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.Video();
                    if (object.ptsUs != null)
                        if ($util.Long)
                            message.ptsUs = $util.Long.fromValue(object.ptsUs, true);
                        else if (typeof object.ptsUs === "string")
                            message.ptsUs = parseInt(object.ptsUs, 10);
                        else if (typeof object.ptsUs === "number")
                            message.ptsUs = object.ptsUs;
                        else if (typeof object.ptsUs === "object")
                            message.ptsUs = new $util.LongBits(object.ptsUs.low >>> 0, object.ptsUs.high >>> 0).toNumber(true);
                    if (object.codec != null)
                        message.codec = object.codec >>> 0;
                    if (object.key != null)
                        message.key = Boolean(object.key);
                    if (object.accessUnit != null)
                        if (typeof object.accessUnit === "string")
                            $util.base64.decode(object.accessUnit, message.accessUnit = $util.newBuffer($util.base64.length(object.accessUnit)), 0);
                        else if (object.accessUnit.length >= 0)
                            message.accessUnit = object.accessUnit;
                    return message;
                };

                /**
                 * Creates a plain object from a Video message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.Video
                 * @static
                 * @param {eyepiece.rpc.v1.Video} message Video
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                Video.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.defaults) {
                        if ($util.Long) {
                            let long = new $util.Long(0, 0, true);
                            object.ptsUs = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : typeof BigInt !== "undefined" && options.longs === BigInt ? long.toBigInt() : long;
                        } else
                            object.ptsUs = options.longs === String ? "0" : typeof BigInt !== "undefined" && options.longs === BigInt ? BigInt("0") : 0;
                        object.codec = 0;
                        object.key = false;
                        if (options.bytes === String)
                            object.accessUnit = "";
                        else {
                            object.accessUnit = [];
                            if (options.bytes !== Array)
                                object.accessUnit = $util.newBuffer(object.accessUnit);
                        }
                    }
                    if (message.ptsUs != null && Object.hasOwnProperty.call(message, "ptsUs"))
                        if (typeof BigInt !== "undefined" && options.longs === BigInt)
                            object.ptsUs = typeof message.ptsUs === "number" ? BigInt(message.ptsUs) : $util.Long.fromBits(message.ptsUs.low >>> 0, message.ptsUs.high >>> 0, true).toBigInt();
                        else if (typeof message.ptsUs === "number")
                            object.ptsUs = options.longs === String ? String(message.ptsUs) : message.ptsUs;
                        else
                            object.ptsUs = options.longs === String ? $util.Long.prototype.toString.call(message.ptsUs) : options.longs === Number ? new $util.LongBits(message.ptsUs.low >>> 0, message.ptsUs.high >>> 0).toNumber(true) : message.ptsUs;
                    if (message.codec != null && Object.hasOwnProperty.call(message, "codec"))
                        object.codec = message.codec;
                    if (message.key != null && Object.hasOwnProperty.call(message, "key"))
                        object.key = message.key;
                    if (message.accessUnit != null && Object.hasOwnProperty.call(message, "accessUnit"))
                        object.accessUnit = options.bytes === String ? $util.base64.encode(message.accessUnit, 0, message.accessUnit.length) : options.bytes === Array ? Array.prototype.slice.call(message.accessUnit) : message.accessUnit;
                    return object;
                };

                /**
                 * Converts this Video to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.Video
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                Video.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for Video
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.Video
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                Video.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.Video";
                };

                return Video;
            })();

            v1.Ping = (function() {

                /**
                 * Properties of a Ping.
                 * @memberof eyepiece.rpc.v1
                 * @interface IPing
                 * @property {number|Long|null} [pcUs] Ping pcUs
                 */

                /**
                 * Constructs a new Ping.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a Ping.
                 * @implements IPing
                 * @constructor
                 * @param {eyepiece.rpc.v1.IPing=} [properties] Properties to set
                 */
                function Ping(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Ping pcUs.
                 * @member {number|Long} pcUs
                 * @memberof eyepiece.rpc.v1.Ping
                 * @instance
                 */
                Ping.prototype.pcUs = $util.Long ? $util.Long.fromBits(0,0,true) : 0;

                /**
                 * Creates a new Ping instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.Ping
                 * @static
                 * @param {eyepiece.rpc.v1.IPing=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.Ping} Ping instance
                 */
                Ping.create = function create(properties) {
                    return new Ping(properties);
                };

                /**
                 * Encodes the specified Ping message. Does not implicitly {@link eyepiece.rpc.v1.Ping.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.Ping
                 * @static
                 * @param {eyepiece.rpc.v1.IPing} message Ping message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Ping.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.pcUs != null && Object.hasOwnProperty.call(message, "pcUs"))
                        writer.uint32(/* id 1, wireType 0 =*/8).uint64(message.pcUs);
                    return writer;
                };

                /**
                 * Encodes the specified Ping message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Ping.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.Ping
                 * @static
                 * @param {eyepiece.rpc.v1.IPing} message Ping message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Ping.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a Ping message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.Ping
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.Ping} Ping
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Ping.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.Ping();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                message.pcUs = reader.uint64();
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a Ping message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.Ping
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.Ping} Ping
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Ping.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a Ping message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.Ping
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                Ping.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.pcUs != null && Object.hasOwnProperty.call(message, "pcUs"))
                        if (!$util.isInteger(message.pcUs) && !(message.pcUs && $util.isInteger(message.pcUs.low) && $util.isInteger(message.pcUs.high)))
                            return "pcUs: integer|Long expected";
                    return null;
                };

                /**
                 * Creates a Ping message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.Ping
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.Ping} Ping
                 */
                Ping.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.Ping)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.Ping: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.Ping();
                    if (object.pcUs != null)
                        if ($util.Long)
                            message.pcUs = $util.Long.fromValue(object.pcUs, true);
                        else if (typeof object.pcUs === "string")
                            message.pcUs = parseInt(object.pcUs, 10);
                        else if (typeof object.pcUs === "number")
                            message.pcUs = object.pcUs;
                        else if (typeof object.pcUs === "object")
                            message.pcUs = new $util.LongBits(object.pcUs.low >>> 0, object.pcUs.high >>> 0).toNumber(true);
                    return message;
                };

                /**
                 * Creates a plain object from a Ping message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.Ping
                 * @static
                 * @param {eyepiece.rpc.v1.Ping} message Ping
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                Ping.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.defaults)
                        if ($util.Long) {
                            let long = new $util.Long(0, 0, true);
                            object.pcUs = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : typeof BigInt !== "undefined" && options.longs === BigInt ? long.toBigInt() : long;
                        } else
                            object.pcUs = options.longs === String ? "0" : typeof BigInt !== "undefined" && options.longs === BigInt ? BigInt("0") : 0;
                    if (message.pcUs != null && Object.hasOwnProperty.call(message, "pcUs"))
                        if (typeof BigInt !== "undefined" && options.longs === BigInt)
                            object.pcUs = typeof message.pcUs === "number" ? BigInt(message.pcUs) : $util.Long.fromBits(message.pcUs.low >>> 0, message.pcUs.high >>> 0, true).toBigInt();
                        else if (typeof message.pcUs === "number")
                            object.pcUs = options.longs === String ? String(message.pcUs) : message.pcUs;
                        else
                            object.pcUs = options.longs === String ? $util.Long.prototype.toString.call(message.pcUs) : options.longs === Number ? new $util.LongBits(message.pcUs.low >>> 0, message.pcUs.high >>> 0).toNumber(true) : message.pcUs;
                    return object;
                };

                /**
                 * Converts this Ping to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.Ping
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                Ping.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for Ping
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.Ping
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                Ping.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.Ping";
                };

                return Ping;
            })();

            v1.Control = (function() {

                /**
                 * Properties of a Control.
                 * @memberof eyepiece.rpc.v1
                 * @interface IControl
                 * @property {number|null} [operation] Control operation
                 * @property {boolean|null} [enabled] Control enabled
                 */

                /**
                 * Constructs a new Control.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a Control.
                 * @implements IControl
                 * @constructor
                 * @param {eyepiece.rpc.v1.IControl=} [properties] Properties to set
                 */
                function Control(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Control operation.
                 * @member {number} operation
                 * @memberof eyepiece.rpc.v1.Control
                 * @instance
                 */
                Control.prototype.operation = 0;

                /**
                 * Control enabled.
                 * @member {boolean} enabled
                 * @memberof eyepiece.rpc.v1.Control
                 * @instance
                 */
                Control.prototype.enabled = false;

                /**
                 * Creates a new Control instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.Control
                 * @static
                 * @param {eyepiece.rpc.v1.IControl=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.Control} Control instance
                 */
                Control.create = function create(properties) {
                    return new Control(properties);
                };

                /**
                 * Encodes the specified Control message. Does not implicitly {@link eyepiece.rpc.v1.Control.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.Control
                 * @static
                 * @param {eyepiece.rpc.v1.IControl} message Control message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Control.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.operation != null && Object.hasOwnProperty.call(message, "operation"))
                        writer.uint32(/* id 1, wireType 0 =*/8).uint32(message.operation);
                    if (message.enabled != null && Object.hasOwnProperty.call(message, "enabled"))
                        writer.uint32(/* id 2, wireType 0 =*/16).bool(message.enabled);
                    return writer;
                };

                /**
                 * Encodes the specified Control message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Control.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.Control
                 * @static
                 * @param {eyepiece.rpc.v1.IControl} message Control message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Control.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a Control message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.Control
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.Control} Control
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Control.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.Control();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                message.operation = reader.uint32();
                                break;
                            }
                        case 2: {
                                message.enabled = reader.bool();
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a Control message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.Control
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.Control} Control
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Control.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a Control message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.Control
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                Control.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.operation != null && Object.hasOwnProperty.call(message, "operation"))
                        if (!$util.isInteger(message.operation))
                            return "operation: integer expected";
                    if (message.enabled != null && Object.hasOwnProperty.call(message, "enabled"))
                        if (typeof message.enabled !== "boolean")
                            return "enabled: boolean expected";
                    return null;
                };

                /**
                 * Creates a Control message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.Control
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.Control} Control
                 */
                Control.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.Control)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.Control: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.Control();
                    if (object.operation != null)
                        message.operation = object.operation >>> 0;
                    if (object.enabled != null)
                        message.enabled = Boolean(object.enabled);
                    return message;
                };

                /**
                 * Creates a plain object from a Control message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.Control
                 * @static
                 * @param {eyepiece.rpc.v1.Control} message Control
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                Control.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.defaults) {
                        object.operation = 0;
                        object.enabled = false;
                    }
                    if (message.operation != null && Object.hasOwnProperty.call(message, "operation"))
                        object.operation = message.operation;
                    if (message.enabled != null && Object.hasOwnProperty.call(message, "enabled"))
                        object.enabled = message.enabled;
                    return object;
                };

                /**
                 * Converts this Control to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.Control
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                Control.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for Control
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.Control
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                Control.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.Control";
                };

                return Control;
            })();

            v1.Pane = (function() {

                /**
                 * Properties of a Pane.
                 * @memberof eyepiece.rpc.v1
                 * @interface IPane
                 * @property {number|null} [x] Pane x
                 * @property {number|null} [y] Pane y
                 * @property {number|null} [width] Pane width
                 * @property {number|null} [height] Pane height
                 * @property {number|null} [format] Pane format
                 */

                /**
                 * Constructs a new Pane.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a Pane.
                 * @implements IPane
                 * @constructor
                 * @param {eyepiece.rpc.v1.IPane=} [properties] Properties to set
                 */
                function Pane(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Pane x.
                 * @member {number} x
                 * @memberof eyepiece.rpc.v1.Pane
                 * @instance
                 */
                Pane.prototype.x = 0;

                /**
                 * Pane y.
                 * @member {number} y
                 * @memberof eyepiece.rpc.v1.Pane
                 * @instance
                 */
                Pane.prototype.y = 0;

                /**
                 * Pane width.
                 * @member {number} width
                 * @memberof eyepiece.rpc.v1.Pane
                 * @instance
                 */
                Pane.prototype.width = 0;

                /**
                 * Pane height.
                 * @member {number} height
                 * @memberof eyepiece.rpc.v1.Pane
                 * @instance
                 */
                Pane.prototype.height = 0;

                /**
                 * Pane format.
                 * @member {number} format
                 * @memberof eyepiece.rpc.v1.Pane
                 * @instance
                 */
                Pane.prototype.format = 0;

                /**
                 * Creates a new Pane instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.Pane
                 * @static
                 * @param {eyepiece.rpc.v1.IPane=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.Pane} Pane instance
                 */
                Pane.create = function create(properties) {
                    return new Pane(properties);
                };

                /**
                 * Encodes the specified Pane message. Does not implicitly {@link eyepiece.rpc.v1.Pane.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.Pane
                 * @static
                 * @param {eyepiece.rpc.v1.IPane} message Pane message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Pane.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.x != null && Object.hasOwnProperty.call(message, "x"))
                        writer.uint32(/* id 1, wireType 0 =*/8).uint32(message.x);
                    if (message.y != null && Object.hasOwnProperty.call(message, "y"))
                        writer.uint32(/* id 2, wireType 0 =*/16).uint32(message.y);
                    if (message.width != null && Object.hasOwnProperty.call(message, "width"))
                        writer.uint32(/* id 3, wireType 0 =*/24).uint32(message.width);
                    if (message.height != null && Object.hasOwnProperty.call(message, "height"))
                        writer.uint32(/* id 4, wireType 0 =*/32).uint32(message.height);
                    if (message.format != null && Object.hasOwnProperty.call(message, "format"))
                        writer.uint32(/* id 5, wireType 0 =*/40).uint32(message.format);
                    return writer;
                };

                /**
                 * Encodes the specified Pane message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Pane.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.Pane
                 * @static
                 * @param {eyepiece.rpc.v1.IPane} message Pane message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Pane.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a Pane message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.Pane
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.Pane} Pane
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Pane.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.Pane();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                message.x = reader.uint32();
                                break;
                            }
                        case 2: {
                                message.y = reader.uint32();
                                break;
                            }
                        case 3: {
                                message.width = reader.uint32();
                                break;
                            }
                        case 4: {
                                message.height = reader.uint32();
                                break;
                            }
                        case 5: {
                                message.format = reader.uint32();
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a Pane message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.Pane
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.Pane} Pane
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Pane.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a Pane message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.Pane
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                Pane.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.x != null && Object.hasOwnProperty.call(message, "x"))
                        if (!$util.isInteger(message.x))
                            return "x: integer expected";
                    if (message.y != null && Object.hasOwnProperty.call(message, "y"))
                        if (!$util.isInteger(message.y))
                            return "y: integer expected";
                    if (message.width != null && Object.hasOwnProperty.call(message, "width"))
                        if (!$util.isInteger(message.width))
                            return "width: integer expected";
                    if (message.height != null && Object.hasOwnProperty.call(message, "height"))
                        if (!$util.isInteger(message.height))
                            return "height: integer expected";
                    if (message.format != null && Object.hasOwnProperty.call(message, "format"))
                        if (!$util.isInteger(message.format))
                            return "format: integer expected";
                    return null;
                };

                /**
                 * Creates a Pane message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.Pane
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.Pane} Pane
                 */
                Pane.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.Pane)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.Pane: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.Pane();
                    if (object.x != null)
                        message.x = object.x >>> 0;
                    if (object.y != null)
                        message.y = object.y >>> 0;
                    if (object.width != null)
                        message.width = object.width >>> 0;
                    if (object.height != null)
                        message.height = object.height >>> 0;
                    if (object.format != null)
                        message.format = object.format >>> 0;
                    return message;
                };

                /**
                 * Creates a plain object from a Pane message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.Pane
                 * @static
                 * @param {eyepiece.rpc.v1.Pane} message Pane
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                Pane.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.defaults) {
                        object.x = 0;
                        object.y = 0;
                        object.width = 0;
                        object.height = 0;
                        object.format = 0;
                    }
                    if (message.x != null && Object.hasOwnProperty.call(message, "x"))
                        object.x = message.x;
                    if (message.y != null && Object.hasOwnProperty.call(message, "y"))
                        object.y = message.y;
                    if (message.width != null && Object.hasOwnProperty.call(message, "width"))
                        object.width = message.width;
                    if (message.height != null && Object.hasOwnProperty.call(message, "height"))
                        object.height = message.height;
                    if (message.format != null && Object.hasOwnProperty.call(message, "format"))
                        object.format = message.format;
                    return object;
                };

                /**
                 * Converts this Pane to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.Pane
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                Pane.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for Pane
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.Pane
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                Pane.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.Pane";
                };

                return Pane;
            })();

            v1.Configure = (function() {

                /**
                 * Properties of a Configure.
                 * @memberof eyepiece.rpc.v1
                 * @interface IConfigure
                 * @property {number|null} [codec] Configure codec
                 * @property {number|null} [layout] Configure layout
                 * @property {eyepiece.rpc.v1.IPane|null} [pane] Configure pane
                 */

                /**
                 * Constructs a new Configure.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a Configure.
                 * @implements IConfigure
                 * @constructor
                 * @param {eyepiece.rpc.v1.IConfigure=} [properties] Properties to set
                 */
                function Configure(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Configure codec.
                 * @member {number} codec
                 * @memberof eyepiece.rpc.v1.Configure
                 * @instance
                 */
                Configure.prototype.codec = 0;

                /**
                 * Configure layout.
                 * @member {number} layout
                 * @memberof eyepiece.rpc.v1.Configure
                 * @instance
                 */
                Configure.prototype.layout = 0;

                /**
                 * Configure pane.
                 * @member {eyepiece.rpc.v1.IPane|null|undefined} pane
                 * @memberof eyepiece.rpc.v1.Configure
                 * @instance
                 */
                Configure.prototype.pane = null;

                // OneOf field names bound to virtual getters and setters
                let $oneOfFields;

                // Virtual OneOf for proto3 optional field
                Object.defineProperty(Configure.prototype, "_pane", {
                    get: $util.oneOfGetter($oneOfFields = ["pane"]),
                    set: $util.oneOfSetter($oneOfFields)
                });

                /**
                 * Creates a new Configure instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.Configure
                 * @static
                 * @param {eyepiece.rpc.v1.IConfigure=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.Configure} Configure instance
                 */
                Configure.create = function create(properties) {
                    return new Configure(properties);
                };

                /**
                 * Encodes the specified Configure message. Does not implicitly {@link eyepiece.rpc.v1.Configure.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.Configure
                 * @static
                 * @param {eyepiece.rpc.v1.IConfigure} message Configure message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Configure.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.codec != null && Object.hasOwnProperty.call(message, "codec"))
                        writer.uint32(/* id 1, wireType 0 =*/8).uint32(message.codec);
                    if (message.layout != null && Object.hasOwnProperty.call(message, "layout"))
                        writer.uint32(/* id 2, wireType 0 =*/16).uint32(message.layout);
                    if (message.pane != null && Object.hasOwnProperty.call(message, "pane"))
                        $root.eyepiece.rpc.v1.Pane.encode(message.pane, writer.uint32(/* id 3, wireType 2 =*/26).fork(), q + 1).ldelim();
                    return writer;
                };

                /**
                 * Encodes the specified Configure message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Configure.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.Configure
                 * @static
                 * @param {eyepiece.rpc.v1.IConfigure} message Configure message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Configure.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a Configure message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.Configure
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.Configure} Configure
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Configure.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.Configure();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                message.codec = reader.uint32();
                                break;
                            }
                        case 2: {
                                message.layout = reader.uint32();
                                break;
                            }
                        case 3: {
                                message.pane = $root.eyepiece.rpc.v1.Pane.decode(reader, reader.uint32(), undefined, long + 1);
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a Configure message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.Configure
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.Configure} Configure
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Configure.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a Configure message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.Configure
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                Configure.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    let properties = {};
                    if (message.codec != null && Object.hasOwnProperty.call(message, "codec"))
                        if (!$util.isInteger(message.codec))
                            return "codec: integer expected";
                    if (message.layout != null && Object.hasOwnProperty.call(message, "layout"))
                        if (!$util.isInteger(message.layout))
                            return "layout: integer expected";
                    if (message.pane != null && Object.hasOwnProperty.call(message, "pane")) {
                        properties._pane = 1;
                        {
                            let error = $root.eyepiece.rpc.v1.Pane.verify(message.pane, long + 1);
                            if (error)
                                return "pane." + error;
                        }
                    }
                    return null;
                };

                /**
                 * Creates a Configure message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.Configure
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.Configure} Configure
                 */
                Configure.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.Configure)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.Configure: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.Configure();
                    if (object.codec != null)
                        message.codec = object.codec >>> 0;
                    if (object.layout != null)
                        message.layout = object.layout >>> 0;
                    if (object.pane != null) {
                        if (!$util.isObject(object.pane))
                            throw TypeError(".eyepiece.rpc.v1.Configure.pane: object expected");
                        message.pane = $root.eyepiece.rpc.v1.Pane.fromObject(object.pane, long + 1);
                    }
                    return message;
                };

                /**
                 * Creates a plain object from a Configure message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.Configure
                 * @static
                 * @param {eyepiece.rpc.v1.Configure} message Configure
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                Configure.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.defaults) {
                        object.codec = 0;
                        object.layout = 0;
                    }
                    if (message.codec != null && Object.hasOwnProperty.call(message, "codec"))
                        object.codec = message.codec;
                    if (message.layout != null && Object.hasOwnProperty.call(message, "layout"))
                        object.layout = message.layout;
                    if (message.pane != null && Object.hasOwnProperty.call(message, "pane")) {
                        object.pane = $root.eyepiece.rpc.v1.Pane.toObject(message.pane, options, q + 1);
                        if (options.oneofs)
                            object._pane = "pane";
                    }
                    return object;
                };

                /**
                 * Converts this Configure to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.Configure
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                Configure.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for Configure
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.Configure
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                Configure.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.Configure";
                };

                return Configure;
            })();

            v1.GuiAlpha = (function() {

                /**
                 * Properties of a GuiAlpha.
                 * @memberof eyepiece.rpc.v1
                 * @interface IGuiAlpha
                 * @property {number|null} [alpha] GuiAlpha alpha
                 */

                /**
                 * Constructs a new GuiAlpha.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a GuiAlpha.
                 * @implements IGuiAlpha
                 * @constructor
                 * @param {eyepiece.rpc.v1.IGuiAlpha=} [properties] Properties to set
                 */
                function GuiAlpha(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * GuiAlpha alpha.
                 * @member {number} alpha
                 * @memberof eyepiece.rpc.v1.GuiAlpha
                 * @instance
                 */
                GuiAlpha.prototype.alpha = 0;

                /**
                 * Creates a new GuiAlpha instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.GuiAlpha
                 * @static
                 * @param {eyepiece.rpc.v1.IGuiAlpha=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.GuiAlpha} GuiAlpha instance
                 */
                GuiAlpha.create = function create(properties) {
                    return new GuiAlpha(properties);
                };

                /**
                 * Encodes the specified GuiAlpha message. Does not implicitly {@link eyepiece.rpc.v1.GuiAlpha.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.GuiAlpha
                 * @static
                 * @param {eyepiece.rpc.v1.IGuiAlpha} message GuiAlpha message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                GuiAlpha.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.alpha != null && Object.hasOwnProperty.call(message, "alpha"))
                        writer.uint32(/* id 2, wireType 0 =*/16).uint32(message.alpha);
                    return writer;
                };

                /**
                 * Encodes the specified GuiAlpha message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.GuiAlpha.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.GuiAlpha
                 * @static
                 * @param {eyepiece.rpc.v1.IGuiAlpha} message GuiAlpha message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                GuiAlpha.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a GuiAlpha message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.GuiAlpha
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.GuiAlpha} GuiAlpha
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                GuiAlpha.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.GuiAlpha();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 2: {
                                message.alpha = reader.uint32();
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a GuiAlpha message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.GuiAlpha
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.GuiAlpha} GuiAlpha
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                GuiAlpha.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a GuiAlpha message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.GuiAlpha
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                GuiAlpha.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.alpha != null && Object.hasOwnProperty.call(message, "alpha"))
                        if (!$util.isInteger(message.alpha))
                            return "alpha: integer expected";
                    return null;
                };

                /**
                 * Creates a GuiAlpha message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.GuiAlpha
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.GuiAlpha} GuiAlpha
                 */
                GuiAlpha.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.GuiAlpha)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.GuiAlpha: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.GuiAlpha();
                    if (object.alpha != null)
                        message.alpha = object.alpha >>> 0;
                    return message;
                };

                /**
                 * Creates a plain object from a GuiAlpha message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.GuiAlpha
                 * @static
                 * @param {eyepiece.rpc.v1.GuiAlpha} message GuiAlpha
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                GuiAlpha.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.defaults)
                        object.alpha = 0;
                    if (message.alpha != null && Object.hasOwnProperty.call(message, "alpha"))
                        object.alpha = message.alpha;
                    return object;
                };

                /**
                 * Converts this GuiAlpha to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.GuiAlpha
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                GuiAlpha.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for GuiAlpha
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.GuiAlpha
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                GuiAlpha.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.GuiAlpha";
                };

                return GuiAlpha;
            })();

            v1.PaneAlpha = (function() {

                /**
                 * Properties of a PaneAlpha.
                 * @memberof eyepiece.rpc.v1
                 * @interface IPaneAlpha
                 * @property {number|null} [alpha] PaneAlpha alpha
                 */

                /**
                 * Constructs a new PaneAlpha.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a PaneAlpha.
                 * @implements IPaneAlpha
                 * @constructor
                 * @param {eyepiece.rpc.v1.IPaneAlpha=} [properties] Properties to set
                 */
                function PaneAlpha(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * PaneAlpha alpha.
                 * @member {number} alpha
                 * @memberof eyepiece.rpc.v1.PaneAlpha
                 * @instance
                 */
                PaneAlpha.prototype.alpha = 0;

                /**
                 * Creates a new PaneAlpha instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.PaneAlpha
                 * @static
                 * @param {eyepiece.rpc.v1.IPaneAlpha=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.PaneAlpha} PaneAlpha instance
                 */
                PaneAlpha.create = function create(properties) {
                    return new PaneAlpha(properties);
                };

                /**
                 * Encodes the specified PaneAlpha message. Does not implicitly {@link eyepiece.rpc.v1.PaneAlpha.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.PaneAlpha
                 * @static
                 * @param {eyepiece.rpc.v1.IPaneAlpha} message PaneAlpha message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                PaneAlpha.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.alpha != null && Object.hasOwnProperty.call(message, "alpha"))
                        writer.uint32(/* id 1, wireType 0 =*/8).uint32(message.alpha);
                    return writer;
                };

                /**
                 * Encodes the specified PaneAlpha message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.PaneAlpha.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.PaneAlpha
                 * @static
                 * @param {eyepiece.rpc.v1.IPaneAlpha} message PaneAlpha message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                PaneAlpha.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a PaneAlpha message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.PaneAlpha
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.PaneAlpha} PaneAlpha
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                PaneAlpha.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.PaneAlpha();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                message.alpha = reader.uint32();
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a PaneAlpha message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.PaneAlpha
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.PaneAlpha} PaneAlpha
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                PaneAlpha.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a PaneAlpha message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.PaneAlpha
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                PaneAlpha.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.alpha != null && Object.hasOwnProperty.call(message, "alpha"))
                        if (!$util.isInteger(message.alpha))
                            return "alpha: integer expected";
                    return null;
                };

                /**
                 * Creates a PaneAlpha message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.PaneAlpha
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.PaneAlpha} PaneAlpha
                 */
                PaneAlpha.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.PaneAlpha)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.PaneAlpha: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.PaneAlpha();
                    if (object.alpha != null)
                        message.alpha = object.alpha >>> 0;
                    return message;
                };

                /**
                 * Creates a plain object from a PaneAlpha message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.PaneAlpha
                 * @static
                 * @param {eyepiece.rpc.v1.PaneAlpha} message PaneAlpha
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                PaneAlpha.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.defaults)
                        object.alpha = 0;
                    if (message.alpha != null && Object.hasOwnProperty.call(message, "alpha"))
                        object.alpha = message.alpha;
                    return object;
                };

                /**
                 * Converts this PaneAlpha to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.PaneAlpha
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                PaneAlpha.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for PaneAlpha
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.PaneAlpha
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                PaneAlpha.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.PaneAlpha";
                };

                return PaneAlpha;
            })();

            v1.Pong = (function() {

                /**
                 * Properties of a Pong.
                 * @memberof eyepiece.rpc.v1
                 * @interface IPong
                 * @property {number|Long|null} [pcUs] Pong pcUs
                 * @property {number|Long|null} [deviceUs] Pong deviceUs
                 */

                /**
                 * Constructs a new Pong.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a Pong.
                 * @implements IPong
                 * @constructor
                 * @param {eyepiece.rpc.v1.IPong=} [properties] Properties to set
                 */
                function Pong(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Pong pcUs.
                 * @member {number|Long} pcUs
                 * @memberof eyepiece.rpc.v1.Pong
                 * @instance
                 */
                Pong.prototype.pcUs = $util.Long ? $util.Long.fromBits(0,0,true) : 0;

                /**
                 * Pong deviceUs.
                 * @member {number|Long} deviceUs
                 * @memberof eyepiece.rpc.v1.Pong
                 * @instance
                 */
                Pong.prototype.deviceUs = $util.Long ? $util.Long.fromBits(0,0,true) : 0;

                /**
                 * Creates a new Pong instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.Pong
                 * @static
                 * @param {eyepiece.rpc.v1.IPong=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.Pong} Pong instance
                 */
                Pong.create = function create(properties) {
                    return new Pong(properties);
                };

                /**
                 * Encodes the specified Pong message. Does not implicitly {@link eyepiece.rpc.v1.Pong.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.Pong
                 * @static
                 * @param {eyepiece.rpc.v1.IPong} message Pong message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Pong.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.pcUs != null && Object.hasOwnProperty.call(message, "pcUs"))
                        writer.uint32(/* id 1, wireType 0 =*/8).uint64(message.pcUs);
                    if (message.deviceUs != null && Object.hasOwnProperty.call(message, "deviceUs"))
                        writer.uint32(/* id 2, wireType 0 =*/16).uint64(message.deviceUs);
                    return writer;
                };

                /**
                 * Encodes the specified Pong message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Pong.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.Pong
                 * @static
                 * @param {eyepiece.rpc.v1.IPong} message Pong message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Pong.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a Pong message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.Pong
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.Pong} Pong
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Pong.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.Pong();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                message.pcUs = reader.uint64();
                                break;
                            }
                        case 2: {
                                message.deviceUs = reader.uint64();
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a Pong message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.Pong
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.Pong} Pong
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Pong.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a Pong message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.Pong
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                Pong.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.pcUs != null && Object.hasOwnProperty.call(message, "pcUs"))
                        if (!$util.isInteger(message.pcUs) && !(message.pcUs && $util.isInteger(message.pcUs.low) && $util.isInteger(message.pcUs.high)))
                            return "pcUs: integer|Long expected";
                    if (message.deviceUs != null && Object.hasOwnProperty.call(message, "deviceUs"))
                        if (!$util.isInteger(message.deviceUs) && !(message.deviceUs && $util.isInteger(message.deviceUs.low) && $util.isInteger(message.deviceUs.high)))
                            return "deviceUs: integer|Long expected";
                    return null;
                };

                /**
                 * Creates a Pong message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.Pong
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.Pong} Pong
                 */
                Pong.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.Pong)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.Pong: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.Pong();
                    if (object.pcUs != null)
                        if ($util.Long)
                            message.pcUs = $util.Long.fromValue(object.pcUs, true);
                        else if (typeof object.pcUs === "string")
                            message.pcUs = parseInt(object.pcUs, 10);
                        else if (typeof object.pcUs === "number")
                            message.pcUs = object.pcUs;
                        else if (typeof object.pcUs === "object")
                            message.pcUs = new $util.LongBits(object.pcUs.low >>> 0, object.pcUs.high >>> 0).toNumber(true);
                    if (object.deviceUs != null)
                        if ($util.Long)
                            message.deviceUs = $util.Long.fromValue(object.deviceUs, true);
                        else if (typeof object.deviceUs === "string")
                            message.deviceUs = parseInt(object.deviceUs, 10);
                        else if (typeof object.deviceUs === "number")
                            message.deviceUs = object.deviceUs;
                        else if (typeof object.deviceUs === "object")
                            message.deviceUs = new $util.LongBits(object.deviceUs.low >>> 0, object.deviceUs.high >>> 0).toNumber(true);
                    return message;
                };

                /**
                 * Creates a plain object from a Pong message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.Pong
                 * @static
                 * @param {eyepiece.rpc.v1.Pong} message Pong
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                Pong.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.defaults) {
                        if ($util.Long) {
                            let long = new $util.Long(0, 0, true);
                            object.pcUs = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : typeof BigInt !== "undefined" && options.longs === BigInt ? long.toBigInt() : long;
                        } else
                            object.pcUs = options.longs === String ? "0" : typeof BigInt !== "undefined" && options.longs === BigInt ? BigInt("0") : 0;
                        if ($util.Long) {
                            let long = new $util.Long(0, 0, true);
                            object.deviceUs = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : typeof BigInt !== "undefined" && options.longs === BigInt ? long.toBigInt() : long;
                        } else
                            object.deviceUs = options.longs === String ? "0" : typeof BigInt !== "undefined" && options.longs === BigInt ? BigInt("0") : 0;
                    }
                    if (message.pcUs != null && Object.hasOwnProperty.call(message, "pcUs"))
                        if (typeof BigInt !== "undefined" && options.longs === BigInt)
                            object.pcUs = typeof message.pcUs === "number" ? BigInt(message.pcUs) : $util.Long.fromBits(message.pcUs.low >>> 0, message.pcUs.high >>> 0, true).toBigInt();
                        else if (typeof message.pcUs === "number")
                            object.pcUs = options.longs === String ? String(message.pcUs) : message.pcUs;
                        else
                            object.pcUs = options.longs === String ? $util.Long.prototype.toString.call(message.pcUs) : options.longs === Number ? new $util.LongBits(message.pcUs.low >>> 0, message.pcUs.high >>> 0).toNumber(true) : message.pcUs;
                    if (message.deviceUs != null && Object.hasOwnProperty.call(message, "deviceUs"))
                        if (typeof BigInt !== "undefined" && options.longs === BigInt)
                            object.deviceUs = typeof message.deviceUs === "number" ? BigInt(message.deviceUs) : $util.Long.fromBits(message.deviceUs.low >>> 0, message.deviceUs.high >>> 0, true).toBigInt();
                        else if (typeof message.deviceUs === "number")
                            object.deviceUs = options.longs === String ? String(message.deviceUs) : message.deviceUs;
                        else
                            object.deviceUs = options.longs === String ? $util.Long.prototype.toString.call(message.deviceUs) : options.longs === Number ? new $util.LongBits(message.deviceUs.low >>> 0, message.deviceUs.high >>> 0).toNumber(true) : message.deviceUs;
                    return object;
                };

                /**
                 * Converts this Pong to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.Pong
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                Pong.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for Pong
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.Pong
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                Pong.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.Pong";
                };

                return Pong;
            })();

            v1.Frame = (function() {

                /**
                 * Properties of a Frame.
                 * @memberof eyepiece.rpc.v1
                 * @interface IFrame
                 * @property {number|Long|null} [ptsUs] Frame ptsUs
                 * @property {number|Long|null} [receivedUs] Frame receivedUs
                 * @property {number|Long|null} [shownUs] Frame shownUs
                 * @property {number|null} [decodeUs] Frame decodeUs
                 * @property {number|null} [convertUs] Frame convertUs
                 * @property {number|null} [queueDepth] Frame queueDepth
                 * @property {number|null} [dropped] Frame dropped
                 */

                /**
                 * Constructs a new Frame.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a Frame.
                 * @implements IFrame
                 * @constructor
                 * @param {eyepiece.rpc.v1.IFrame=} [properties] Properties to set
                 */
                function Frame(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Frame ptsUs.
                 * @member {number|Long} ptsUs
                 * @memberof eyepiece.rpc.v1.Frame
                 * @instance
                 */
                Frame.prototype.ptsUs = $util.Long ? $util.Long.fromBits(0,0,true) : 0;

                /**
                 * Frame receivedUs.
                 * @member {number|Long} receivedUs
                 * @memberof eyepiece.rpc.v1.Frame
                 * @instance
                 */
                Frame.prototype.receivedUs = $util.Long ? $util.Long.fromBits(0,0,true) : 0;

                /**
                 * Frame shownUs.
                 * @member {number|Long} shownUs
                 * @memberof eyepiece.rpc.v1.Frame
                 * @instance
                 */
                Frame.prototype.shownUs = $util.Long ? $util.Long.fromBits(0,0,true) : 0;

                /**
                 * Frame decodeUs.
                 * @member {number} decodeUs
                 * @memberof eyepiece.rpc.v1.Frame
                 * @instance
                 */
                Frame.prototype.decodeUs = 0;

                /**
                 * Frame convertUs.
                 * @member {number} convertUs
                 * @memberof eyepiece.rpc.v1.Frame
                 * @instance
                 */
                Frame.prototype.convertUs = 0;

                /**
                 * Frame queueDepth.
                 * @member {number} queueDepth
                 * @memberof eyepiece.rpc.v1.Frame
                 * @instance
                 */
                Frame.prototype.queueDepth = 0;

                /**
                 * Frame dropped.
                 * @member {number} dropped
                 * @memberof eyepiece.rpc.v1.Frame
                 * @instance
                 */
                Frame.prototype.dropped = 0;

                /**
                 * Creates a new Frame instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.Frame
                 * @static
                 * @param {eyepiece.rpc.v1.IFrame=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.Frame} Frame instance
                 */
                Frame.create = function create(properties) {
                    return new Frame(properties);
                };

                /**
                 * Encodes the specified Frame message. Does not implicitly {@link eyepiece.rpc.v1.Frame.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.Frame
                 * @static
                 * @param {eyepiece.rpc.v1.IFrame} message Frame message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Frame.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.ptsUs != null && Object.hasOwnProperty.call(message, "ptsUs"))
                        writer.uint32(/* id 1, wireType 0 =*/8).uint64(message.ptsUs);
                    if (message.receivedUs != null && Object.hasOwnProperty.call(message, "receivedUs"))
                        writer.uint32(/* id 2, wireType 0 =*/16).uint64(message.receivedUs);
                    if (message.shownUs != null && Object.hasOwnProperty.call(message, "shownUs"))
                        writer.uint32(/* id 3, wireType 0 =*/24).uint64(message.shownUs);
                    if (message.decodeUs != null && Object.hasOwnProperty.call(message, "decodeUs"))
                        writer.uint32(/* id 4, wireType 0 =*/32).uint32(message.decodeUs);
                    if (message.convertUs != null && Object.hasOwnProperty.call(message, "convertUs"))
                        writer.uint32(/* id 5, wireType 0 =*/40).uint32(message.convertUs);
                    if (message.queueDepth != null && Object.hasOwnProperty.call(message, "queueDepth"))
                        writer.uint32(/* id 6, wireType 0 =*/48).uint32(message.queueDepth);
                    if (message.dropped != null && Object.hasOwnProperty.call(message, "dropped"))
                        writer.uint32(/* id 7, wireType 0 =*/56).uint32(message.dropped);
                    return writer;
                };

                /**
                 * Encodes the specified Frame message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Frame.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.Frame
                 * @static
                 * @param {eyepiece.rpc.v1.IFrame} message Frame message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Frame.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a Frame message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.Frame
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.Frame} Frame
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Frame.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.Frame();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                message.ptsUs = reader.uint64();
                                break;
                            }
                        case 2: {
                                message.receivedUs = reader.uint64();
                                break;
                            }
                        case 3: {
                                message.shownUs = reader.uint64();
                                break;
                            }
                        case 4: {
                                message.decodeUs = reader.uint32();
                                break;
                            }
                        case 5: {
                                message.convertUs = reader.uint32();
                                break;
                            }
                        case 6: {
                                message.queueDepth = reader.uint32();
                                break;
                            }
                        case 7: {
                                message.dropped = reader.uint32();
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a Frame message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.Frame
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.Frame} Frame
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Frame.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a Frame message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.Frame
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                Frame.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.ptsUs != null && Object.hasOwnProperty.call(message, "ptsUs"))
                        if (!$util.isInteger(message.ptsUs) && !(message.ptsUs && $util.isInteger(message.ptsUs.low) && $util.isInteger(message.ptsUs.high)))
                            return "ptsUs: integer|Long expected";
                    if (message.receivedUs != null && Object.hasOwnProperty.call(message, "receivedUs"))
                        if (!$util.isInteger(message.receivedUs) && !(message.receivedUs && $util.isInteger(message.receivedUs.low) && $util.isInteger(message.receivedUs.high)))
                            return "receivedUs: integer|Long expected";
                    if (message.shownUs != null && Object.hasOwnProperty.call(message, "shownUs"))
                        if (!$util.isInteger(message.shownUs) && !(message.shownUs && $util.isInteger(message.shownUs.low) && $util.isInteger(message.shownUs.high)))
                            return "shownUs: integer|Long expected";
                    if (message.decodeUs != null && Object.hasOwnProperty.call(message, "decodeUs"))
                        if (!$util.isInteger(message.decodeUs))
                            return "decodeUs: integer expected";
                    if (message.convertUs != null && Object.hasOwnProperty.call(message, "convertUs"))
                        if (!$util.isInteger(message.convertUs))
                            return "convertUs: integer expected";
                    if (message.queueDepth != null && Object.hasOwnProperty.call(message, "queueDepth"))
                        if (!$util.isInteger(message.queueDepth))
                            return "queueDepth: integer expected";
                    if (message.dropped != null && Object.hasOwnProperty.call(message, "dropped"))
                        if (!$util.isInteger(message.dropped))
                            return "dropped: integer expected";
                    return null;
                };

                /**
                 * Creates a Frame message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.Frame
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.Frame} Frame
                 */
                Frame.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.Frame)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.Frame: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.Frame();
                    if (object.ptsUs != null)
                        if ($util.Long)
                            message.ptsUs = $util.Long.fromValue(object.ptsUs, true);
                        else if (typeof object.ptsUs === "string")
                            message.ptsUs = parseInt(object.ptsUs, 10);
                        else if (typeof object.ptsUs === "number")
                            message.ptsUs = object.ptsUs;
                        else if (typeof object.ptsUs === "object")
                            message.ptsUs = new $util.LongBits(object.ptsUs.low >>> 0, object.ptsUs.high >>> 0).toNumber(true);
                    if (object.receivedUs != null)
                        if ($util.Long)
                            message.receivedUs = $util.Long.fromValue(object.receivedUs, true);
                        else if (typeof object.receivedUs === "string")
                            message.receivedUs = parseInt(object.receivedUs, 10);
                        else if (typeof object.receivedUs === "number")
                            message.receivedUs = object.receivedUs;
                        else if (typeof object.receivedUs === "object")
                            message.receivedUs = new $util.LongBits(object.receivedUs.low >>> 0, object.receivedUs.high >>> 0).toNumber(true);
                    if (object.shownUs != null)
                        if ($util.Long)
                            message.shownUs = $util.Long.fromValue(object.shownUs, true);
                        else if (typeof object.shownUs === "string")
                            message.shownUs = parseInt(object.shownUs, 10);
                        else if (typeof object.shownUs === "number")
                            message.shownUs = object.shownUs;
                        else if (typeof object.shownUs === "object")
                            message.shownUs = new $util.LongBits(object.shownUs.low >>> 0, object.shownUs.high >>> 0).toNumber(true);
                    if (object.decodeUs != null)
                        message.decodeUs = object.decodeUs >>> 0;
                    if (object.convertUs != null)
                        message.convertUs = object.convertUs >>> 0;
                    if (object.queueDepth != null)
                        message.queueDepth = object.queueDepth >>> 0;
                    if (object.dropped != null)
                        message.dropped = object.dropped >>> 0;
                    return message;
                };

                /**
                 * Creates a plain object from a Frame message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.Frame
                 * @static
                 * @param {eyepiece.rpc.v1.Frame} message Frame
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                Frame.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.defaults) {
                        if ($util.Long) {
                            let long = new $util.Long(0, 0, true);
                            object.ptsUs = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : typeof BigInt !== "undefined" && options.longs === BigInt ? long.toBigInt() : long;
                        } else
                            object.ptsUs = options.longs === String ? "0" : typeof BigInt !== "undefined" && options.longs === BigInt ? BigInt("0") : 0;
                        if ($util.Long) {
                            let long = new $util.Long(0, 0, true);
                            object.receivedUs = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : typeof BigInt !== "undefined" && options.longs === BigInt ? long.toBigInt() : long;
                        } else
                            object.receivedUs = options.longs === String ? "0" : typeof BigInt !== "undefined" && options.longs === BigInt ? BigInt("0") : 0;
                        if ($util.Long) {
                            let long = new $util.Long(0, 0, true);
                            object.shownUs = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : typeof BigInt !== "undefined" && options.longs === BigInt ? long.toBigInt() : long;
                        } else
                            object.shownUs = options.longs === String ? "0" : typeof BigInt !== "undefined" && options.longs === BigInt ? BigInt("0") : 0;
                        object.decodeUs = 0;
                        object.convertUs = 0;
                        object.queueDepth = 0;
                        object.dropped = 0;
                    }
                    if (message.ptsUs != null && Object.hasOwnProperty.call(message, "ptsUs"))
                        if (typeof BigInt !== "undefined" && options.longs === BigInt)
                            object.ptsUs = typeof message.ptsUs === "number" ? BigInt(message.ptsUs) : $util.Long.fromBits(message.ptsUs.low >>> 0, message.ptsUs.high >>> 0, true).toBigInt();
                        else if (typeof message.ptsUs === "number")
                            object.ptsUs = options.longs === String ? String(message.ptsUs) : message.ptsUs;
                        else
                            object.ptsUs = options.longs === String ? $util.Long.prototype.toString.call(message.ptsUs) : options.longs === Number ? new $util.LongBits(message.ptsUs.low >>> 0, message.ptsUs.high >>> 0).toNumber(true) : message.ptsUs;
                    if (message.receivedUs != null && Object.hasOwnProperty.call(message, "receivedUs"))
                        if (typeof BigInt !== "undefined" && options.longs === BigInt)
                            object.receivedUs = typeof message.receivedUs === "number" ? BigInt(message.receivedUs) : $util.Long.fromBits(message.receivedUs.low >>> 0, message.receivedUs.high >>> 0, true).toBigInt();
                        else if (typeof message.receivedUs === "number")
                            object.receivedUs = options.longs === String ? String(message.receivedUs) : message.receivedUs;
                        else
                            object.receivedUs = options.longs === String ? $util.Long.prototype.toString.call(message.receivedUs) : options.longs === Number ? new $util.LongBits(message.receivedUs.low >>> 0, message.receivedUs.high >>> 0).toNumber(true) : message.receivedUs;
                    if (message.shownUs != null && Object.hasOwnProperty.call(message, "shownUs"))
                        if (typeof BigInt !== "undefined" && options.longs === BigInt)
                            object.shownUs = typeof message.shownUs === "number" ? BigInt(message.shownUs) : $util.Long.fromBits(message.shownUs.low >>> 0, message.shownUs.high >>> 0, true).toBigInt();
                        else if (typeof message.shownUs === "number")
                            object.shownUs = options.longs === String ? String(message.shownUs) : message.shownUs;
                        else
                            object.shownUs = options.longs === String ? $util.Long.prototype.toString.call(message.shownUs) : options.longs === Number ? new $util.LongBits(message.shownUs.low >>> 0, message.shownUs.high >>> 0).toNumber(true) : message.shownUs;
                    if (message.decodeUs != null && Object.hasOwnProperty.call(message, "decodeUs"))
                        object.decodeUs = message.decodeUs;
                    if (message.convertUs != null && Object.hasOwnProperty.call(message, "convertUs"))
                        object.convertUs = message.convertUs;
                    if (message.queueDepth != null && Object.hasOwnProperty.call(message, "queueDepth"))
                        object.queueDepth = message.queueDepth;
                    if (message.dropped != null && Object.hasOwnProperty.call(message, "dropped"))
                        object.dropped = message.dropped;
                    return object;
                };

                /**
                 * Converts this Frame to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.Frame
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                Frame.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for Frame
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.Frame
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                Frame.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.Frame";
                };

                return Frame;
            })();

            v1.Info = (function() {

                /**
                 * Properties of an Info.
                 * @memberof eyepiece.rpc.v1
                 * @interface IInfo
                 * @property {number|null} [version] Info version
                 * @property {number|null} [framebufferWidth] Info framebufferWidth
                 * @property {number|null} [framebufferHeight] Info framebufferHeight
                 * @property {number|null} [decoder] Info decoder
                 * @property {number|null} [headtrack] Info headtrack
                 * @property {number|null} [flags] Info flags
                 */

                /**
                 * Constructs a new Info.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents an Info.
                 * @implements IInfo
                 * @constructor
                 * @param {eyepiece.rpc.v1.IInfo=} [properties] Properties to set
                 */
                function Info(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Info version.
                 * @member {number} version
                 * @memberof eyepiece.rpc.v1.Info
                 * @instance
                 */
                Info.prototype.version = 0;

                /**
                 * Info framebufferWidth.
                 * @member {number} framebufferWidth
                 * @memberof eyepiece.rpc.v1.Info
                 * @instance
                 */
                Info.prototype.framebufferWidth = 0;

                /**
                 * Info framebufferHeight.
                 * @member {number} framebufferHeight
                 * @memberof eyepiece.rpc.v1.Info
                 * @instance
                 */
                Info.prototype.framebufferHeight = 0;

                /**
                 * Info decoder.
                 * @member {number} decoder
                 * @memberof eyepiece.rpc.v1.Info
                 * @instance
                 */
                Info.prototype.decoder = 0;

                /**
                 * Info headtrack.
                 * @member {number} headtrack
                 * @memberof eyepiece.rpc.v1.Info
                 * @instance
                 */
                Info.prototype.headtrack = 0;

                /**
                 * Info flags.
                 * @member {number} flags
                 * @memberof eyepiece.rpc.v1.Info
                 * @instance
                 */
                Info.prototype.flags = 0;

                /**
                 * Creates a new Info instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.Info
                 * @static
                 * @param {eyepiece.rpc.v1.IInfo=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.Info} Info instance
                 */
                Info.create = function create(properties) {
                    return new Info(properties);
                };

                /**
                 * Encodes the specified Info message. Does not implicitly {@link eyepiece.rpc.v1.Info.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.Info
                 * @static
                 * @param {eyepiece.rpc.v1.IInfo} message Info message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Info.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.version != null && Object.hasOwnProperty.call(message, "version"))
                        writer.uint32(/* id 1, wireType 0 =*/8).uint32(message.version);
                    if (message.framebufferWidth != null && Object.hasOwnProperty.call(message, "framebufferWidth"))
                        writer.uint32(/* id 2, wireType 0 =*/16).uint32(message.framebufferWidth);
                    if (message.framebufferHeight != null && Object.hasOwnProperty.call(message, "framebufferHeight"))
                        writer.uint32(/* id 3, wireType 0 =*/24).uint32(message.framebufferHeight);
                    if (message.decoder != null && Object.hasOwnProperty.call(message, "decoder"))
                        writer.uint32(/* id 4, wireType 0 =*/32).uint32(message.decoder);
                    if (message.headtrack != null && Object.hasOwnProperty.call(message, "headtrack"))
                        writer.uint32(/* id 5, wireType 0 =*/40).uint32(message.headtrack);
                    if (message.flags != null && Object.hasOwnProperty.call(message, "flags"))
                        writer.uint32(/* id 6, wireType 0 =*/48).uint32(message.flags);
                    return writer;
                };

                /**
                 * Encodes the specified Info message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Info.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.Info
                 * @static
                 * @param {eyepiece.rpc.v1.IInfo} message Info message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Info.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes an Info message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.Info
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.Info} Info
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Info.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.Info();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                message.version = reader.uint32();
                                break;
                            }
                        case 2: {
                                message.framebufferWidth = reader.uint32();
                                break;
                            }
                        case 3: {
                                message.framebufferHeight = reader.uint32();
                                break;
                            }
                        case 4: {
                                message.decoder = reader.uint32();
                                break;
                            }
                        case 5: {
                                message.headtrack = reader.uint32();
                                break;
                            }
                        case 6: {
                                message.flags = reader.uint32();
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes an Info message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.Info
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.Info} Info
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Info.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies an Info message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.Info
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                Info.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.version != null && Object.hasOwnProperty.call(message, "version"))
                        if (!$util.isInteger(message.version))
                            return "version: integer expected";
                    if (message.framebufferWidth != null && Object.hasOwnProperty.call(message, "framebufferWidth"))
                        if (!$util.isInteger(message.framebufferWidth))
                            return "framebufferWidth: integer expected";
                    if (message.framebufferHeight != null && Object.hasOwnProperty.call(message, "framebufferHeight"))
                        if (!$util.isInteger(message.framebufferHeight))
                            return "framebufferHeight: integer expected";
                    if (message.decoder != null && Object.hasOwnProperty.call(message, "decoder"))
                        if (!$util.isInteger(message.decoder))
                            return "decoder: integer expected";
                    if (message.headtrack != null && Object.hasOwnProperty.call(message, "headtrack"))
                        if (!$util.isInteger(message.headtrack))
                            return "headtrack: integer expected";
                    if (message.flags != null && Object.hasOwnProperty.call(message, "flags"))
                        if (!$util.isInteger(message.flags))
                            return "flags: integer expected";
                    return null;
                };

                /**
                 * Creates an Info message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.Info
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.Info} Info
                 */
                Info.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.Info)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.Info: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.Info();
                    if (object.version != null)
                        message.version = object.version >>> 0;
                    if (object.framebufferWidth != null)
                        message.framebufferWidth = object.framebufferWidth >>> 0;
                    if (object.framebufferHeight != null)
                        message.framebufferHeight = object.framebufferHeight >>> 0;
                    if (object.decoder != null)
                        message.decoder = object.decoder >>> 0;
                    if (object.headtrack != null)
                        message.headtrack = object.headtrack >>> 0;
                    if (object.flags != null)
                        message.flags = object.flags >>> 0;
                    return message;
                };

                /**
                 * Creates a plain object from an Info message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.Info
                 * @static
                 * @param {eyepiece.rpc.v1.Info} message Info
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                Info.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.defaults) {
                        object.version = 0;
                        object.framebufferWidth = 0;
                        object.framebufferHeight = 0;
                        object.decoder = 0;
                        object.headtrack = 0;
                        object.flags = 0;
                    }
                    if (message.version != null && Object.hasOwnProperty.call(message, "version"))
                        object.version = message.version;
                    if (message.framebufferWidth != null && Object.hasOwnProperty.call(message, "framebufferWidth"))
                        object.framebufferWidth = message.framebufferWidth;
                    if (message.framebufferHeight != null && Object.hasOwnProperty.call(message, "framebufferHeight"))
                        object.framebufferHeight = message.framebufferHeight;
                    if (message.decoder != null && Object.hasOwnProperty.call(message, "decoder"))
                        object.decoder = message.decoder;
                    if (message.headtrack != null && Object.hasOwnProperty.call(message, "headtrack"))
                        object.headtrack = message.headtrack;
                    if (message.flags != null && Object.hasOwnProperty.call(message, "flags"))
                        object.flags = message.flags;
                    return object;
                };

                /**
                 * Converts this Info to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.Info
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                Info.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for Info
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.Info
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                Info.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.Info";
                };

                return Info;
            })();

            v1.KeyRequest = (function() {

                /**
                 * Properties of a KeyRequest.
                 * @memberof eyepiece.rpc.v1
                 * @interface IKeyRequest
                 * @property {number|null} [reason] KeyRequest reason
                 */

                /**
                 * Constructs a new KeyRequest.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents a KeyRequest.
                 * @implements IKeyRequest
                 * @constructor
                 * @param {eyepiece.rpc.v1.IKeyRequest=} [properties] Properties to set
                 */
                function KeyRequest(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * KeyRequest reason.
                 * @member {number} reason
                 * @memberof eyepiece.rpc.v1.KeyRequest
                 * @instance
                 */
                KeyRequest.prototype.reason = 0;

                /**
                 * Creates a new KeyRequest instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.KeyRequest
                 * @static
                 * @param {eyepiece.rpc.v1.IKeyRequest=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.KeyRequest} KeyRequest instance
                 */
                KeyRequest.create = function create(properties) {
                    return new KeyRequest(properties);
                };

                /**
                 * Encodes the specified KeyRequest message. Does not implicitly {@link eyepiece.rpc.v1.KeyRequest.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.KeyRequest
                 * @static
                 * @param {eyepiece.rpc.v1.IKeyRequest} message KeyRequest message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                KeyRequest.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.reason != null && Object.hasOwnProperty.call(message, "reason"))
                        writer.uint32(/* id 1, wireType 0 =*/8).uint32(message.reason);
                    return writer;
                };

                /**
                 * Encodes the specified KeyRequest message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.KeyRequest.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.KeyRequest
                 * @static
                 * @param {eyepiece.rpc.v1.IKeyRequest} message KeyRequest message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                KeyRequest.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes a KeyRequest message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.KeyRequest
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.KeyRequest} KeyRequest
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                KeyRequest.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.KeyRequest();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 1: {
                                message.reason = reader.uint32();
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes a KeyRequest message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.KeyRequest
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.KeyRequest} KeyRequest
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                KeyRequest.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a KeyRequest message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.KeyRequest
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                KeyRequest.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.reason != null && Object.hasOwnProperty.call(message, "reason"))
                        if (!$util.isInteger(message.reason))
                            return "reason: integer expected";
                    return null;
                };

                /**
                 * Creates a KeyRequest message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.KeyRequest
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.KeyRequest} KeyRequest
                 */
                KeyRequest.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.KeyRequest)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.KeyRequest: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.KeyRequest();
                    if (object.reason != null)
                        message.reason = object.reason >>> 0;
                    return message;
                };

                /**
                 * Creates a plain object from a KeyRequest message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.KeyRequest
                 * @static
                 * @param {eyepiece.rpc.v1.KeyRequest} message KeyRequest
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                KeyRequest.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.defaults)
                        object.reason = 0;
                    if (message.reason != null && Object.hasOwnProperty.call(message, "reason"))
                        object.reason = message.reason;
                    return object;
                };

                /**
                 * Converts this KeyRequest to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.KeyRequest
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                KeyRequest.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for KeyRequest
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.KeyRequest
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                KeyRequest.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.KeyRequest";
                };

                return KeyRequest;
            })();

            v1.Acknowledgement = (function() {

                /**
                 * Properties of an Acknowledgement.
                 * @memberof eyepiece.rpc.v1
                 * @interface IAcknowledgement
                 * @property {number|null} [status] Acknowledgement status
                 */

                /**
                 * Constructs a new Acknowledgement.
                 * @memberof eyepiece.rpc.v1
                 * @classdesc Represents an Acknowledgement.
                 * @implements IAcknowledgement
                 * @constructor
                 * @param {eyepiece.rpc.v1.IAcknowledgement=} [properties] Properties to set
                 */
                function Acknowledgement(properties) {
                    if (properties)
                        for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null && keys[i] !== "__proto__")
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * Acknowledgement status.
                 * @member {number} status
                 * @memberof eyepiece.rpc.v1.Acknowledgement
                 * @instance
                 */
                Acknowledgement.prototype.status = 0;

                /**
                 * Creates a new Acknowledgement instance using the specified properties.
                 * @function create
                 * @memberof eyepiece.rpc.v1.Acknowledgement
                 * @static
                 * @param {eyepiece.rpc.v1.IAcknowledgement=} [properties] Properties to set
                 * @returns {eyepiece.rpc.v1.Acknowledgement} Acknowledgement instance
                 */
                Acknowledgement.create = function create(properties) {
                    return new Acknowledgement(properties);
                };

                /**
                 * Encodes the specified Acknowledgement message. Does not implicitly {@link eyepiece.rpc.v1.Acknowledgement.verify|verify} messages.
                 * @function encode
                 * @memberof eyepiece.rpc.v1.Acknowledgement
                 * @static
                 * @param {eyepiece.rpc.v1.IAcknowledgement} message Acknowledgement message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Acknowledgement.encode = function encode(message, writer, q) {
                    if (!writer)
                        writer = $Writer.create();
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    if (message.status != null && Object.hasOwnProperty.call(message, "status"))
                        writer.uint32(/* id 2, wireType 0 =*/16).uint32(message.status);
                    return writer;
                };

                /**
                 * Encodes the specified Acknowledgement message, length delimited. Does not implicitly {@link eyepiece.rpc.v1.Acknowledgement.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof eyepiece.rpc.v1.Acknowledgement
                 * @static
                 * @param {eyepiece.rpc.v1.IAcknowledgement} message Acknowledgement message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                Acknowledgement.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer && writer.len ? writer.fork() : writer).ldelim();
                };

                /**
                 * Decodes an Acknowledgement message from the specified reader or buffer.
                 * @function decode
                 * @memberof eyepiece.rpc.v1.Acknowledgement
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {eyepiece.rpc.v1.Acknowledgement} Acknowledgement
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Acknowledgement.decode = function decode(reader, length, error, long) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    if (long === undefined)
                        long = 0;
                    if (long > $Reader.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let end, message;
                    if (length === undefined)
                        end = reader.len;
                    else {
                        end = reader.pos + length;
                        if (end > reader.len)
                            throw RangeError("index out of range");
                        length = reader.len;
                        reader.len = end;
                    }
                    message = new $root.eyepiece.rpc.v1.Acknowledgement();
                    while (reader.pos < end) {
                        let tag = reader.uint32();
                        if (tag === error)
                            break;
                        switch (tag >>> 3) {
                        case 2: {
                                message.status = reader.uint32();
                                break;
                            }
                        default:
                            reader.skipType(tag & 7, long);
                            break;
                        }
                    }
                    if (length !== undefined) {
                        if (reader.pos !== end)
                            throw RangeError("index out of range");
                        reader.len = length;
                    }
                    return message;
                };

                /**
                 * Decodes an Acknowledgement message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof eyepiece.rpc.v1.Acknowledgement
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {eyepiece.rpc.v1.Acknowledgement} Acknowledgement
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                Acknowledgement.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies an Acknowledgement message.
                 * @function verify
                 * @memberof eyepiece.rpc.v1.Acknowledgement
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                Acknowledgement.verify = function verify(message, long) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        return "maximum nesting depth exceeded";
                    if (message.status != null && Object.hasOwnProperty.call(message, "status"))
                        if (!$util.isInteger(message.status))
                            return "status: integer expected";
                    return null;
                };

                /**
                 * Creates an Acknowledgement message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof eyepiece.rpc.v1.Acknowledgement
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {eyepiece.rpc.v1.Acknowledgement} Acknowledgement
                 */
                Acknowledgement.fromObject = function fromObject(object, long) {
                    if (object instanceof $root.eyepiece.rpc.v1.Acknowledgement)
                        return object;
                    if (!$util.isObject(object))
                        throw TypeError(".eyepiece.rpc.v1.Acknowledgement: object expected");
                    if (long === undefined)
                        long = 0;
                    if (long > $util.recursionLimit)
                        throw Error("maximum nesting depth exceeded");
                    let message = new $root.eyepiece.rpc.v1.Acknowledgement();
                    if (object.status != null)
                        message.status = object.status >>> 0;
                    return message;
                };

                /**
                 * Creates a plain object from an Acknowledgement message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof eyepiece.rpc.v1.Acknowledgement
                 * @static
                 * @param {eyepiece.rpc.v1.Acknowledgement} message Acknowledgement
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                Acknowledgement.toObject = function toObject(message, options, q) {
                    if (!options)
                        options = {};
                    if (q === undefined)
                        q = 0;
                    if (q > $util.recursionLimit)
                        throw Error("max depth exceeded");
                    let object = {};
                    if (options.defaults)
                        object.status = 0;
                    if (message.status != null && Object.hasOwnProperty.call(message, "status"))
                        object.status = message.status;
                    return object;
                };

                /**
                 * Converts this Acknowledgement to JSON.
                 * @function toJSON
                 * @memberof eyepiece.rpc.v1.Acknowledgement
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                Acknowledgement.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                /**
                 * Gets the default type url for Acknowledgement
                 * @function getTypeUrl
                 * @memberof eyepiece.rpc.v1.Acknowledgement
                 * @static
                 * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
                 * @returns {string} The default type url
                 */
                Acknowledgement.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                    if (typeUrlPrefix === undefined) {
                        typeUrlPrefix = "type.googleapis.com";
                    }
                    return typeUrlPrefix + "/eyepiece.rpc.v1.Acknowledgement";
                };

                return Acknowledgement;
            })();

            return v1;
        })();

        return rpc;
    })();

    return eyepiece;
})();

export { $root as default };
