# html/browsers/windows/browsing-context-names/choose-_top-002.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/browsing-context-names/choose-_top-002.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>HTML Test: Browsing context name - _top</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/PrefixedLocalStorage.js"></script>
<div id="log"></div>
<script>
var prefixedStorage;
setup (() => prefixedStorage = new PrefixedLocalStorageTest());

async_test(t => {
  t.add_cleanup(() => prefixedStorage.cleanup());

  var testFunc = (function (t) {
    var completed = 0;
    var testCount = 2;
    return function (actual, expected) {
      assert_equals(actual, expected);
      if (++completed >= testCount) {
        t.done();
      }
    }
  }(t));

  prefixedStorage.onSet('isTop', t.step_func(e => {
    testFunc(e.newValue, 'true');
  }));
  prefixedStorage.onSet('name', t.step_func(e => {
    testFunc(e.newValue, 'topWin2');
  }));
  window.open(prefixedStorage.url('resources/choose-_top-002-window.html'), '_blank');
}, 'Should choose top browsing context for "_top" if current is not top');
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
  "source_name": "html/browsers/windows/browsing-context-names/choose-_top-002.html"
}
```
