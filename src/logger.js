import { invoke } from "@tauri-apps/api";

const logf = async (level, format, ...args) => {

    let index = 0;

    const result = format.replace(/{}/g, (match) => {
        if (index >= args.length) {
            throw new Error('too few arguments.');
        }

        const arg = args[index++];

        switch (typeof arg) {
            case 'undefined': return 'undefined';
            case 'object': return !arg ? 'null' : JSON.stringify(arg);
            case 'boolean': return arg ? 'true' : 'false';
            case 'number': 
            case 'bigint': return String(arg);
            case 'string': return arg;
            case 'symbol': return arg.toString();
            case 'function': return arg.toString();
        }

    });

    return invoke('fe_logger', {lvl: level, msg: result});
};

const trace = async (format, ...args) => {
    return logf("trace", format, args);
};

const debug = async (format, ...args) => {
    return logf("debug", format, args);
};

const info = async (format, ...args) => {
    return logf("info", format, args);
};

const warn = async (format, ...args) => {
    return logf("warn", format, args);
};

const error = async (format, ...args) => {
    return logf("error", format, args);
};

export {trace, debug, info, warn, error};
