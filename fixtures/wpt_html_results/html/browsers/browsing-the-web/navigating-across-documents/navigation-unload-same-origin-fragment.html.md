# html/browsers/browsing-the-web/navigating-across-documents/navigation-unload-same-origin-fragment.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/navigation-unload-same-origin-fragment.html",
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
<link rel="help" href="https://html.spec.whatwg.org/multipage/browsers.html#navigating-across-documents">
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">

<iframe id="i" src="navigation-unload-same-origin-fragment-1.html"></iframe>

<!-- a timeout indicates that setting i.contentWindow.location.hash (a second navigation) aborted the first navigation,
    and so it stayed on a.html and finishedLoading was never called -->

<script>
var test = async_test('Tests that a fragment navigation in the unload handler will not block the initial navigation');
window.onload = test.step_func(function() {
    var i = document.querySelector('#i');

    i.contentWindow.onunload = test.step_func(function() {
        i.contentWindow.location.hash = '#fragment';
    });

    window.finishedLoading = test.step_func_done(function () {
        assert_equals(i.contentWindow.location.pathname.split('/').pop(), 'navigation-unload-same-origin-fragment-2.html');
        assert_equals(i.contentWindow.location.hash, '');
    });

    i.contentWindow.location.href = 'navigation-unload-same-origin-fragment-2.html';
});
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/navigation-unload-same-origin-fragment.html"
}
```
