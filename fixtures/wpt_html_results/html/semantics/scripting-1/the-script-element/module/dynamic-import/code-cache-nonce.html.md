# html/semantics/scripting-1/the-script-element/module/dynamic-import/code-cache-nonce.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/dynamic-import/code-cache-nonce.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
// Regression test for https://crbug.com/979351:
// This test loads a same script file (`resources/code-cache-nonce.js`)
// with three different nonces (i.e. different host defined options).
// Dynamic imports from the script therefore should have different nonces,
// but when code caching ignores the difference in nonces, the first nonce
// ('abc') is reused incorrectly for subsequent dynamic imports, causing
// CSP violation (and thus dynamic import rejection).

function runTest(nonce, description) {
  // Perform a dynamic import with nonce=`nonce`
  // from a page (`iframe`) with a matching CSP script-src 'nonce-`nonce`'.
  // This should be successful.
  promise_test(t => {
    return new Promise((resolve, reject) => {
      const iframe = document.createElement('iframe');
      iframe.src = 'resources/code-cache-nonce-iframe.sub.html?nonce=' + nonce;
      iframe.onload = () => {
        // `globalThis.promise` is set by `resources/code-cache-nonce.js`.
        // `t.step_timeout()` is workaround for https://crbug.com/1247801.
        globalThis.promise.then(
          v => t.step_timeout(() => resolve(v), 0),
          v => t.step_timeout(() => reject(v), 0)
        );
      };
      document.body.appendChild(iframe);
      t.add_cleanup(() => iframe.remove());
    });
  }, description);
}

// As `promise_test` are serialized, each iframe is created after previous
// iframes and scripts are completely loaded.
runTest('abc', 'First dynamic import should use nonce=abc');
runTest('def', 'Second dynamic import should use nonce=def');
runTest('ghi', 'Third dynamic import should use nonce=ghi');
</script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/dynamic-import/code-cache-nonce.html"
}
```
