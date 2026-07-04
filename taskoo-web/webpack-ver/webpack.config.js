const path = require('path');
var HtmlWebpackPlugin = require('html-webpack-plugin');

const backendUrl = process.env.TASKOO_BACKEND_URL || 'http://127.0.0.1:7001';

module.exports = {
  mode: 'development',
  entry: './src/index.js',
  output: {
    filename: 'main.js',
    path: path.resolve(__dirname, 'dist'),
  },
  devServer: {
    static: './dist',
    host: '0.0.0.0',
    port: 4141,
    allowedHosts: 'all',
    headers : {
      'X-Forwarded-Proto': 'https'
    },
    proxy: {
      '/api': {
        target: backendUrl,
        changeOrigin: true,
        pathRewrite: {'^/api': ''},
        secure: false
      }
    }
  },
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: ['style-loader', 'css-loader'],
      },
    ],
  },
};
