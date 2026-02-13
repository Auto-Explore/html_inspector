# html/browsers/windows/auxiliary-browsing-contexts/opener-multiple.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/auxiliary-browsing-contexts/opener-multiple.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Auxiliary Browsing Contexts: window.opener, multiple</title>
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
      a.href      = prefixedLocalStorage.url('resources/multiple-opener.html');
      a.target    = 'multipleOpener';
      window.name = 'topOpener';
      document.body.appendChild(a);
      window.addEventListener('message', t.step_func_done(e => {
        var aux1 = e.data.aux1; // First opened context
        var aux2 = e.data.aux2; // Context opened by first-opened context
        assert_equals(aux1.name, 'multipleOpener');
        assert_equals(aux1.openerName, window.name);
        assert_equals(aux1.isTop, true);
        assert_equals(aux2.name, 'multipleOpenee');
        assert_equals(aux2.openerName, aux1.name);
        assert_equals(aux2.isTop, true);
      }));
      a.click();
    }, 'An auxiliary browsing context should be able to open another auxiliary browsing context');
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
  "source_name": "html/browsers/windows/auxiliary-browsing-contexts/opener-multiple.html"
}
```
