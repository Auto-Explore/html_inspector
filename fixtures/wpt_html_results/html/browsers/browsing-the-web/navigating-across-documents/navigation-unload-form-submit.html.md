# html/browsers/browsing-the-web/navigating-across-documents/navigation-unload-form-submit.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/navigation-unload-form-submit.html",
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

<iframe id="i" src="navigation-unload-form-submit-1.html"></iframe>

<!-- derived from https://bugzilla.mozilla.org/show_bug.cgi?id=247660#c0 -->

<script>
var test = async_test('Tests that navigation during an unload caused by a form submit does nothing');
window.onload = test.step_func(function() {
    var i = document.querySelector('#i');

    window.finishedLoading = test.step_func_done(function () {
        assert_equals(i.contentWindow.location.pathname.split('/').pop(), 'navigation-unload-form-submit-2.html');
        assert_equals(i.contentWindow.location.hash, '');
    });

    i.contentWindow.document.querySelector('input[type="submit"]').click();
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/navigation-unload-form-submit.html"
}
```
