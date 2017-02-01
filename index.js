#!/usr/bin/env node

var host = process.env.APP_HOST || 'localhost';
var port = 3000;

var bundle = require('./bundler.js');
bundle();

var express = require('express');
var httpProxy = require('http-proxy');
var proxy = httpProxy.createProxyServer();
var app = express();

app.all(['/assets/*', '*.hot-update.json'], function(req, res) {
    proxy.web(req, res, {
        target: 'http://' + host + ':3001'
    });
});

app.all('/*', function(req, res) {
    proxy.web(req, res, {
        target: 'http://' + host + ':6767'
    });
});

app.listen(port, function() {
    console.log('Server running on port ' + port);
});
