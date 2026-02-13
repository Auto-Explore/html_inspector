# html/browsers/windows/auxiliary-browsing-contexts/opener-closed.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/auxiliary-browsing-contexts/opener-closed.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<html>
  <head>
    <title>Auxiliary Browsing Contexts: window.opener when Opener Removed/Closed</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <script src="/common/PrefixedLocalStorage.js"></script>
  </head>
  <body>
    <div id="log"></div>
    <script>
    var prefixedLocalStorage;
    setup (() => prefixedLocalStorage = new PrefixedLocalStorageTest());
    async_test(t => {
      t.add_cleanup (() => prefixedLocalStorage.cleanup());
      var a       = document.createElement('a');
      a.href      = prefixedLocalStorage.url('resources/open-closer.html');
      a.target    = '_blank';
      prefixedLocalStorage.onSet('openerIsNull', t.step_func_done(e => {
        // The window for this auxiliary browsing context's opener
        // has been closed and discarded, so the aux browsing context
        // should now report `null` for `window.opener`
        assert_equals(e.newValue, 'true');
      }));
      document.body.append(a);
      a.click();
    }, 'An auxiliary browsing context should report `null` for `window.opener` when that browsing context is discarded');
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
  "source_name": "html/browsers/windows/auxiliary-browsing-contexts/opener-closed.html"
}
```
