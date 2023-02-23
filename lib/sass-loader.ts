import * as pkg from "../package.json"; // assert { type: "json" };
import { compile, compileLegacy } from "..";

import type { SassError, SassResult, LegacySassOptions, SassOptions } from "..";

// options
type LoggerOptions = {
  deprecation: boolean;
  span: { url: string; start: { line: number; column: number } };
  stack: string;
};

type Options = SassOptions & {
  importers: { canonicalize: () => Promise<void>; load: () => void }[];
  logger: {
    debug: (message: string, options: LoggerOptions) => void;
    warn: (message: string, options: LoggerOptions) => void;
  };
  url: URL;
};

type LegacyOptions = LegacySassOptions & {
  importer: ((originalUrl: string, prev: any, done: any) => void)[];
  logger: {
    debug: (message: string, options: LoggerOptions) => void;
    warn: (message: string, options: LoggerOptions) => void;
  };
};

const info = `dart-sass\t${pkg.version}`;

const compileStringAsync = async (
  source: string,
  options?: Options
): Promise<SassResult> => {
  const ret = compile(
    source,
    options ? { ...options, file: options.file.toString() } : undefined
  );

  if (ret.success) {
    return ret.success;
  }

  throw ret.failure;
};

const render = (
  options: LegacyOptions,
  callback: (error?: SassError, result?: SassResult) => void
): void => {
  const ret = compileLegacy(options.data, options);

  if (ret.success) {
    return callback(undefined, ret.success);
  }

  return callback(ret.failure, undefined);
};

export { info, compileStringAsync, render };
