# html/browsers/windows/browsing-context-names/choose-_blank-002.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/browsing-context-names/choose-_blank-002.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Link with target=_blank, rel=noreferrer</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/PrefixedLocalStorage.js"></script>
<div id="log"></div>
<a href="resources/report-has-opener.html" rel="noreferrer" target="_blank">Link</a>
<script>
var prefixedStorage;
setup (() => prefixedStorage = new PrefixedLocalStorageTest());

async_test(t => {
  t.add_cleanup(() => prefixedStorage.cleanup());
  var a = document.getElementsByTagName('a')[0];
  a.href = prefixedStorage.url(a.href);
  prefixedStorage.onSet('hasOpener', t.step_func_done(e => {
    assert_equals(e.newValue, 'false');
  }));
  a.click();
}, 'Context for opened noreferrer link targeted to "_blank" should not have opener reference');
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
  "source_name": "html/browsers/windows/browsing-context-names/choose-_blank-002.html"
}
```
