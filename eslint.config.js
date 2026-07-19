const pluginSecurity = require("eslint-plugin-security");

module.exports = [
  {
    ignores: [
      "**/node_modules/**",
      "**/target/**",
      "**/docs/book/**",
      "**/*.min.js",
      "**/mermaid*.js",
    ],
  },
  pluginSecurity.configs.recommended,
  {
    rules: {
      "security/detect-object-injection": "error",
    },
  },
];
