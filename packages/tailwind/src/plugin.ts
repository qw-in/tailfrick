import { withOptions } from "tailwindcss/plugin";
import { run, default as webInit } from "@tailfrick/core";
import type { CSSRuleObject } from "tailwindcss/types/config";

// @note defined by esbuilt
declare const IS_NODE: boolean;

if (!IS_NODE) {
  // @todo not so brittle
  console.debug('[tailfrick] running init')
  console.debug(`[tailfrick] ${import.meta.url}`);
  await webInit({ module_or_path: "https://unpkg.com/@tailfrick/core/pkg/tailfrick-web_bg.wasm" });
}

export type TailfrickPluginOptions = {};

const ERROR_RULE: CSSRuleObject = {
  position: 'relative',
  "&::after": {
    content: "'skill issue'",
    position: "absolute",
    top: "50%",
    left: "50%",
    transform: "translate(-50%, -50%)",
    "background-color": "red",
    color: "white",
    padding: "4px 12px",
    "border-radius": "9999px",
    "font-size": "12px",
    "font-weight": "bold",
    "white-space": "nowrap",
  },
};

export const tailfrick = withOptions<Partial<TailfrickPluginOptions>>(
  (_options = {}) =>
    ({ matchUtilities }) => {
      matchUtilities(
        {
          frick: (value) => {
            try {
              const result = run(value);
              const rule: CSSRuleObject = {};

              for (const property of result.split(";")) {
                const [key, value] = property.split(":");
                rule[key] = value;
              }

              return rule;
            } catch (cause) {
              return ERROR_RULE;
            }
          },
        },
        { values: {} }
      );
    }
);

export default tailfrick;
