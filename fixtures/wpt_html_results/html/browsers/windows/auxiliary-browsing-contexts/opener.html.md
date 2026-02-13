# html/browsers/windows/auxiliary-browsing-contexts/opener.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/auxiliary-browsing-contexts/opener.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Auxiliary Browsing Contexts: window.opener</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <script src="/common/PrefixedLocalStorage.js"></script>
  </head>
  <body>
    <div id="log"></div>
    <script>
    var prefixedLocalStorage;
    setup (() => {
      window.name = 'topWindow';
      prefixedLocalStorage = new PrefixedLocalStorageTest();
    });

    function cleanup () {
      prefixedLocalStorage.setItem('closeAll', 'true');
      prefixedLocalStorage.clear();
    }

    function testOpener (t, target) {
      t.add_cleanup(cleanup);
      window.addEventListener('message', t.step_func(e => {
        if (e.data.name === target) {
          // The opener IDL attribute...must return the WindowProxy object of the
          // browsing context from which the current browsing context was created
          assert_equals(e.data.openerName, 'topWindow');
          // Auxiliary browsing contexts are always top-level browsing contexts
          assert_equals(e.data.isTop, true);
          t.done();
        }
      }));
    }

    async_test(t => {
      var target = 'windowOpenerA';
      var a      = document.createElement('a');
      a.href     = prefixedLocalStorage.url('resources/message-window-opener.html');
      a.target   = target;
      document.body.appendChild(a);
      testOpener(t, target);
      a.click();
    }, 'Newly-created auxiliary browsing context should report `window.opener`');

    async_test(t => {
      var target = 'windowOpenerB';
      testOpener(t, target);
      window.open(prefixedLocalStorage.url('resources/message-window-opener.html'),
        target);
    }, 'Browsing context created with `window.open` should report `window.opener`');
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
  "source_name": "html/browsers/windows/auxiliary-browsing-contexts/opener.html"
}
```
