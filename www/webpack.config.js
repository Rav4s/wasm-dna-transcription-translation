const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  devtool: 'eval-source-map',
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html'])
  ],
};
