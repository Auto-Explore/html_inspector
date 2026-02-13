# html/webappapis/system-state-and-capabilities/the-navigator-object/protocol.tentative.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/system-state-and-capabilities/the-navigator-object/protocol.tentative.https.html",
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

<script>
// This should be merged into protocol.https.html when/if
// https://github.com/whatwg/html/pull/5482 is approved.
[
  'cabal',
  'dat',
  'did',
  'dweb',
  'ethereum',
  'hyper',
  'ipfs',
  'ipns',
  'ssb',
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
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/webappapis/system-state-and-capabilities/the-navigator-object/protocol.tentative.https.html"
}
```
