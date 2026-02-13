# html/browsers/windows/auxiliary-browsing-contexts/opener-setter.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/auxiliary-browsing-contexts/opener-setter.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Auxiliary Browsing Contexts: window.opener setter</title>
    <meta name=timeout content=long>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <script src="/common/PrefixedLocalStorage.js"></script>
  </head>
  <body>
    <div id="log"></div>
    <script>
    var prefixedLocalStorage;
    setup(() => prefixedLocalStorage = new PrefixedLocalStorageTest());
    async_test(t => {
      t.add_cleanup(() => prefixedLocalStorage.cleanup());
      prefixedLocalStorage.onSet('openerIsNull', t.step_func_done(e => {
        assert_equals(e.newValue, 'true');
      }));
      window.open(prefixedLocalStorage.url('resources/opener-setter.html'),
        'iShouldSetOpenerToNull');
    }, 'Auxiliary browsing context created via `window.open` and setting `window.opener` to `null` should report `window.opener` `null`');
    async_test(t => {
      t.add_cleanup(() => prefixedLocalStorage.cleanup());
      prefixedLocalStorage.onSet('openerIsTest', t.step_func_done(e => {
        assert_equals(e.newValue, 'true');
      }));
      window.open(prefixedLocalStorage.url('resources/opener-setter.html'),
        'iShouldSetOpenerToTest');
    }, 'Auxiliary browsing context created via `window.open` and setting `window.opener` to `test` should report `test`');
    </script>
  </body>
</html>
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
  "source_name": "html/browsers/windows/auxiliary-browsing-contexts/opener-setter.html"
}
```
