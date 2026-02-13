# html/webappapis/system-state-and-capabilities/the-navigator-object/protocol.https.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/system-state-and-capabilities/the-navigator-object/protocol.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset='utf-8'>
<title>protocol handlers</title>

<script src='/resources/testharness.js'></script>
<script src='/resources/testharnessreport.js'></script>

<noscript><p>Enable JavaScript and reload.</p></noscript>

<p><strong>Note:</strong> If your browser limits the number of handler
registration requests on a page, you might need to disable or significantly
increase that limit for the tests below to run.</p>

<script>
test(() => {
  assert_idl_attribute(navigator, 'registerProtocolHandler');
}, 'the registerProtocolHandler method should exist on the navigator object');

test(() => {
  assert_idl_attribute(navigator, 'unregisterProtocolHandler');
}, 'the unregisterProtocolHandler method should exist on the navigator object');

/* URL argument */
[
  '%s',
  'foo/%s',
  `%s${location.href}`,
  location.href.replace(location.protocol,
    `${location.protocol[0]}%s${location.protocol.substring(1)}`),
  location.href.replace(location.protocol, `${location.protocol}%s`),
  location.href + '/%s',
  location.href + '#%s',
  location.href + '?foo=%s',
  location.href + '?foo=%s&bar',
  location.href + '/%s/bar/baz/',
  location.href + '/%s/bar/baz/?foo=1337&bar#baz',
  location.href + '/%s/foo/%s/',
].forEach(url => {
  test(() => {
    navigator.registerProtocolHandler('tel', url, 'foo');
  }, 'registerProtocolHandler: Valid URL "' + url + '" should work.');

  test(() => {
    navigator.unregisterProtocolHandler('tel', url);
  }, 'unregisterProtocolHandler: Valid URL "' + url + '" should work.');
});

/* Invalid URLs */
[
  '',
  '%S',
  'http://%s.com',
  'http://%s.example.com',
  location.href.replace(location.hostname, `%s${location.hostname}`),
  location.href.replace(location.port, `%s${location.port}`),
  location.href + '',
  location.href + '/%',
  location.href + '/%a',
  'http://example.com',
  'http://[v8.:::]//url=%s',
  'https://test:test/',
].forEach(url => {
  test(() => {
    assert_throws_dom('SYNTAX_ERR', () => { navigator.registerProtocolHandler('mailto', url, 'foo'); });
    assert_throws_dom('SECURITY_ERR', () => { navigator.registerProtocolHandler('x', url, 'foo'); });
  }, `registerProtocolHandler: Invalid URL "${url}" should throw (but after scheme)`);

  test(() => {
    assert_throws_dom('SYNTAX_ERR', () => { navigator.unregisterProtocolHandler('mailto', url); });
    assert_throws_dom('SECURITY_ERR', () => { navigator.unregisterProtocolHandler('x', url, 'foo'); });
  }, `unregisterProtocolHandler: Invalid URL "${url}" should throw (but after scheme)`);
});

[
  'http://example.com/%s',
  'https://example.com/%s',
  'http://foobar.example.com/%s',
  'mailto:%s@example.com',
  'mailto:%s',
  `chrome://${location.host}/%s`,
  `foo://${location.host}/%s`,
  URL.createObjectURL(new Blob()) + "#%s",
].forEach(url => {
  const title = url.startsWith("blob:") ? "blob: URL" : url;
  test(() => {
    assert_throws_dom('SECURITY_ERR', () => { navigator.registerProtocolHandler('mailto', url, 'foo'); });
  }, `registerProtocolHandler: Invalid URL "${title}" should throw SECURITY_ERR.`);

  test(() => {
    assert_throws_dom('SECURITY_ERR', () => { navigator.unregisterProtocolHandler('mailto', url); });
  }, `unregisterProtocolHandler: Invalid URL "${title}" should throw SECURITY_ERR.`);
});

/* Protocol argument */

/* Overriding any of the following protocols must never be allowed. That would
 * break the browser. */
[
  'about',
  'attachment',
  'blob',
  'chrome',
  'cid',
  'data',
  'file',
  'http',
  'https',
  'javascript',
  'livescript',
  'mid',
  'mocha',
  'moz-icon',
  'opera',
  'operamail',
  'res',
  'resource',
  'shttp',
  'tcl',
  'vbscript',
  'view-source',
  'ws',
  'wss',
  'wyciwyg',
  /* other invalid schemes */
  'unrecognized',
  'mаilto', /* a cyrillic "а" */
  'mailto:',
  'mailto://',
  'mailto' + String.fromCharCode(0),
  'mailtoo' + String.fromCharCode(8),
  'mailto' + String.fromCharCode(10),
  'http://',
  'ssh:/',
  'magnet:+',
  'tel:sip',
  'foo',
  'fweb+oo',
  /* web+ prefixed schemes must be followed by 1+ ascii alphas */
  'web+',
  'web+1',
  'web+namewithid123',
  'web+namewithtrailingspace ',
  'web+préfixewithaccent',     // é is not ascii alpha
  'web+Kelvinsign',            // ASCII-lower KELVIN SIGN is not k
  'web+latinsmallletterlongſ', // ASCII-lower LATIN SMALL LETTER LONG S is not s
  'web+dots.are.forbidden',
  'web+dashes-are-forbidden',
  'web+underscores_are_forbidden',
  'web+spaces are forbidden',
  'web+non*alpha*are*forbidden',
  'web+digits123areforbidden',
].forEach(scheme => {
  test(() => {
    // https://test:test/ does not parse and does not contain %s, but the scheme check happens first
    assert_throws_dom('SECURITY_ERR', () => { navigator.registerProtocolHandler(scheme, 'https://test:test/', 'foo'); });
  }, 'registerProtocolHandler: Attempting to override the "' + scheme + '" protocol should throw SECURITY_ERR.');

  test(() => {
    assert_throws_dom('SECURITY_ERR', () => { navigator.unregisterProtocolHandler(scheme, 'https://test:test/'); });
  }, 'unregisterProtocolHandler: Attempting to override the "' + scheme + '" protocol should throw SECURITY_ERR.');
});

/* The following protocols must be possible to override.
 * We're just testing that the call goes through here. Whether or not they
 * actually work as handlers is covered by the interactive tests. */

[
  /* safelisted schemes listed in
   * https://html.spec.whatwg.org/multipage/system-state.html#safelisted-scheme */
  'bitcoin',
  'ftp',
  'ftps',
  'geo',
  'im',
  'irc',
  'ircs',
  'magnet',
  'mailto',
  'matrix',
  'mms',
  'news',
  'nntp',
  'openpgp4fpr',
  'sftp',
  'sip',
  'sms',
  'smsto',
  'ssh',
  'tel',
  'urn',
  'webcal',
  'wtai',
  'xmpp',
  /* other valid schemes */
  'BitcoIn',
  'Irc',
  'MagneT',
  'Matrix',
  'SmsTo',
  'TEL',
  'teL',
  'WebCAL',
  'WTAI',
  'web+myprotocol',
  'web+abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ', // all alphas
  'web+UpperCasedIsLowercased',
  'WEB+seeabove',
  'WeB+SeEaBoVe'
].forEach(scheme => {
  test(() => {
    navigator.registerProtocolHandler(scheme, location.href + '/%s', "foo");
  }, 'registerProtocolHandler: overriding the "' + scheme + '" protocol should work');

  test(() => {
    navigator.unregisterProtocolHandler(scheme, location.href + '/%s');
  }, 'unregisterProtocolHandler: overriding the "' + scheme + '" protocol should work');
});
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.not_nfc",
      "message": "Text run is not in Unicode Normalization Form C. Should instead be “\ntest(() => {\n  assert_idl_attribute(navigator, 'registerProtocolHandler');\n}, 'the registerProtocolHandler method should exist on the navigator object');\n\ntest(() => {\n  assert_idl_attribute(navigator, 'unregisterProtocolHandler');\n}, 'the unregisterProtocolHandler method should exist on the navigator object');\n\n/* URL argument */\n[\n  '%s',\n  'foo/%s',\n  `%s${location.href}`,\n  location.href.replace(location.protocol,\n    `${location.protocol[0]}%s${location.protocol.substring(1)}`),\n  location.href.replace(location.protocol, `${location.protocol}%s`),\n  location.href + '/%s',\n  location.href + '#%s',\n  location.href + '?foo=%s',\n  location.href + '?foo=%s&bar',\n  location.href + '/%s/bar/baz/',\n  location.href + '/%s/bar/baz/?foo=1337&bar#baz',\n  location.href + '/%s/foo/%s/',\n].forEach(url => {\n  test(() => {\n    navigator.registerProtocolHandler('tel', url, 'foo');\n  }, 'registerProtocolHandler: Valid URL \"' + url + '\" should work.');\n\n  test(() => {\n    navigator.unregisterProtocolHandler('tel', url);\n  }, 'unregisterProtocolHandler: Valid URL \"' + url + '\" should work.');\n});\n\n/* Invalid URLs */\n[\n  '',\n  '%S',\n  'http://%s.com',\n  'http://%s.example.com',\n  location.href.replace(location.hostname, `%s${location.hostname}`),\n  location.href.replace(location.port, `%s${location.port}`),\n  location.href + '',\n  location.href + '/%',\n  location.href + '/%a',\n  'http://example.com',\n  'http://[v8.:::]//url=%s',\n  'https://test:test/',\n].forEach(url => {\n  test(() => {\n    assert_throws_dom('SYNTAX_ERR', () => { navigator.registerProtocolHandler('mailto', url, 'foo'); });\n    assert_throws_dom('SECURITY_ERR', () => { navigator.registerProtocolHandler('x', url, 'foo'); });\n  }, `registerProtocolHandler: Invalid URL \"${url}\" should throw (but after scheme)`);\n\n  test(() => {\n    assert_throws_dom('SYNTAX_ERR', () => { navigator.unregisterProtocolHandler('mailto', url); });\n    assert_throws_dom('SECURITY_ERR', () => { navigator.unregisterProtocolHandler('x', url, 'foo'); });\n  }, `unregisterProtocolHandler: Invalid URL \"${url}\" should throw (but after scheme)`);\n});\n\n[\n  'http://example.com/%s',\n  'https://example.com/%s',\n  'http://foobar.example.com/%s',\n  'mailto:%s@example.com',\n  'mailto:%s',\n  `chrome://${location.host}/%s`,\n  `foo://${location.host}/%s`,\n  URL.createObjectURL(new Blob()) + \"#%s\",\n].forEach(url => {\n  const title = url.startsWith(\"blob:\") ? \"blob: URL\" : url;\n  test(() => {\n    assert_throws_dom('SECURITY_ERR', () => { navigator.registerProtocolHandler('mailto', url, 'foo'); });\n  }, `registerProtocolHandler: Invalid URL \"${title}\" should throw SECURITY_ERR.`);\n\n  test(() => {\n    assert_throws_dom('SECURITY_ERR', () => { navigator.unregisterProtocolHandler('mailto', url); });\n  }, `unregisterProtocolHandler: Invalid URL \"${title}\" should throw SECURITY_ERR.`);\n});\n\n/* Protocol argument */\n\n/* Overriding any of the following protocols must never be allowed. That would\n * break the browser. */\n[\n  'about',\n  'attachment',\n  'blob',\n  'chrome',\n  'cid',\n  'data',\n  'file',\n  'http',\n  'https',\n  'javascript',\n  'livescript',\n  'mid',\n  'mocha',\n  'moz-icon',\n  'opera',\n  'operamail',\n  'res',\n  'resource',\n  'shttp',\n  'tcl',\n  'vbscript',\n  'view-source',\n  'ws',\n  'wss',\n  'wyciwyg',\n  /* other invalid schemes */\n  'unrecognized',\n  'mаilto', /* a cyrillic \"а\" */\n  'mailto:',\n  'mailto://',\n  'mailto' + String.fromCharCode(0),\n  'mailtoo' + String.fromCharCode(8),\n  'mailto' + String.fromCharCode(10),\n  'http://',\n  'ssh:/',\n  'magnet:+',\n  'tel:sip',\n  'foo',\n  'fweb+oo',\n  /* web+ prefixed schemes must be followed by 1+ ascii alphas */\n  'web+',\n  'web+1',\n  'web+namewithid123',\n  'web+namewithtrailingspace ',\n  'web+préfixewithaccent',     // é is not ascii alpha\n  'web+Kelvinsign',            // ASCII-lower KELVIN SIGN is not k\n  'web+latinsmallletterlongſ', // ASCII-lower LATIN SMALL LETTER LONG S is not s\n  'web+dots.are.forbidden',\n  'web+dashes-are-forbidden',\n  'web+underscores_are_forbidden',\n  'web+spaces are forbidden',\n  'web+non*alpha*are*forbidden',\n  'web+digits123areforbidden',\n].forEach(scheme => {\n  test(() => {\n    // https://test:test/ does not parse and does not contain %s, but the scheme check happens first\n    assert_throws_dom('SECURITY_ERR', () => { navigator.registerProtocolHandler(scheme, 'https://test:test/', 'foo'); });\n  }, 'registerProtocolHandler: Attempting to override the \"' + scheme + '\" protocol should throw SECURITY_ERR.');\n\n  test(() => {\n    assert_throws_dom('SECURITY_ERR', () => { navigator.unregisterProtocolHandler(scheme, 'https://test:test/'); });\n  }, 'unregisterProtocolHandler: Attempting to override the \"' + scheme + '\" protocol should throw SECURITY_ERR.');\n});\n\n/* The following protocols must be possible to override.\n * We're just testing that the call goes through here. Whether or not they\n * actually work as handlers is covered by the interactive tests. */\n\n[\n  /* safelisted schemes listed in\n   * https://html.spec.whatwg.org/multipage/system-state.html#safelisted-scheme */\n  'bitcoin',\n  'ftp',\n  'ftps',\n  'geo',\n  'im',\n  'irc',\n  'ircs',\n  'magnet',\n  'mailto',\n  'matrix',\n  'mms',\n  'news',\n  'nntp',\n  'openpgp4fpr',\n  'sftp',\n  'sip',\n  'sms',\n  'smsto',\n  'ssh',\n  'tel',\n  'urn',\n  'webcal',\n  'wtai',\n  'xmpp',\n  /* other valid schemes */\n  'BitcoIn',\n  'Irc',\n  'MagneT',\n  'Matrix',\n  'SmsTo',\n  'TEL',\n  'teL',\n  'WebCAL',\n  'WTAI',\n  'web+myprotocol',\n  'web+abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ', // all alphas\n  'web+UpperCasedIsLowercased',\n  'WEB+seeabove',\n  'WeB+SeEaBoVe'\n].forEach(scheme => {\n  test(() => {\n    navigator.registerProtocolHandler(scheme, location.href + '/%s', \"foo\");\n  }, 'registerProtocolHandler: overriding the \"' + scheme + '\" protocol should work');\n\n  test(() => {\n    navigator.unregisterProtocolHandler(scheme, location.href + '/%s');\n  }, 'unregisterProtocolHandler: overriding the \"' + scheme + '\" protocol should work');\n});”. (Copy and paste that into your source document to replace the un-normalized text.)",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/webappapis/system-state-and-capabilities/the-navigator-object/protocol.https.html"
}
```
