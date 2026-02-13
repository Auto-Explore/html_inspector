# html/browsers/windows/auxiliary-browsing-contexts/opener-noreferrer.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/auxiliary-browsing-contexts/opener-noreferrer.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Auxiliary Browsing Contexts: window.opener noreferrer</title>
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
      var a       = document.createElement('a');
      a.href      = prefixedLocalStorage.url('resources/no-opener.html');
      a.target    = '_blank';
      a.rel       = 'noreferrer';
      window.name = 'topWindow';
      document.body.appendChild(a);
      prefixedLocalStorage.onSet('openerIsNull', t.step_func_done(e => {
        assert_equals(e.newValue, 'true');
      }));
      a.click();
    }, 'Auxiliary browsing context created with `rel="noreferrer"` should report `window.opener` `null`');
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
  "source_name": "html/browsers/windows/auxiliary-browsing-contexts/opener-noreferrer.html"
}
```
