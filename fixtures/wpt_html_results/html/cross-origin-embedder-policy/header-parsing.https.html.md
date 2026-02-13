# html/cross-origin-embedder-policy/header-parsing.https.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-embedder-policy/header-parsing.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <meta charset="utf-8">
  <meta name="timeout" content="long">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
<body>
<script>
'use strict';
function createIframe(t, values) {
  const parent = document.createElement('iframe');
  const child = document.createElement('iframe');
  const params = values.map((value) => {
    const percentEncodedValue = typeof value === "object" ? value.percentEncoded : encodeURIComponent(value);
    return `value=${percentEncodedValue}`;
  });
  parent.setAttribute('src', `resources/empty-coep.py?${params.join("&")}`);
  document.body.appendChild(parent);
  t.add_cleanup(() => parent.remove());

  return new Promise((resolve, reject) => {
      parent.onload = resolve;
      parent.onerror = () =>
        reject(new Error(`failed to load from ${parent.src}`));
    })
    .then(() => {
        child.setAttribute('src', '/common/blank.html');
        parent.contentDocument.body.appendChild(child);
        return new Promise((resolve) => {
            child.onload = resolve;
            child.onerror = () =>
              reject(new Error(`failed to load from ${child.src}`));
          });
      })
    .then(() => child);
}

[
  [],
  [''],
  ['jibberish'],
  [{ percentEncoded: 'require%FFcorp' }], // non-ASCII byte
  ['require-corp;'],
  ['\u000brequire-corp\u000b'], // vertical tab
  ['\u000crequire-corp\u000c'], // form feed
  ['\u000drequire-corp\u000d'], // carriage return
  ['Require-corp'],
  ['"require-corp"'], // HTTP structured header "string" item
  [':cmVxdWlyZS1jb3Jw:'], // HTTP structured header "byte sequence" item
  ['require-corp;\tfoo=bar'],
  ['require-corp require-corp'],
  ['require-corp,require-corp'],
  ['require-corp', 'require-corp'],
  ['', 'require-corp'],
  ['require-corp', ''],
].forEach((values) => {
  promise_test((t) => {
    return createIframe(t, values)
      .then((child) => {
          assert_not_equals(child.contentDocument, null);
        });
  }, 'navigation allowed for ' + JSON.stringify(values));
});

[
  ['require-corp'],
  [' require-corp '],
  ['\trequire-corp\t'], // leading and trailing OWS is not part of the field-value per HTTP
  [' \trequire-corp'],
  ['require-corp\t '],
  ['require-corp; foo=bar'],
  ['require-corp;require-corp'],
  ['require-corp; report-to="data:', '"'], // `require-corp; report-to="data:, "`

].forEach((values) => {
  promise_test((t) => {
    return createIframe(t, values)
      .then((child) => {
          assert_equals(child.contentDocument, null);
        });
  }, 'navigation blocked for ' + JSON.stringify(values));
});
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
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
  "source_name": "html/cross-origin-embedder-policy/header-parsing.https.html"
}
```
