# html/browsers/browsing-the-web/navigating-across-documents/javascript-url-abort/javascript-url-abort-return-value-undefined.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/javascript-url-abort/javascript-url-abort-return-value-undefined.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Not aborting fetch for javascript:undefined navigation</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#navigate">
<link rel="help" href="https://github.com/whatwg/html/issues/2590">
<div id="log"></div>
<iframe src="support/iframe-and-links.html"></iframe>
<script>
async_test(test => {
  onload = () => {
    const child = document.querySelector('iframe').contentWindow;
    child.document.querySelector("#slowLink").click();
    // The step below is in a timeout. The framed page can't communicate back mid-parse because that
    // would involve running script, which makes that navigation "mature", and we need to do this
    // before it matures.
    test.step_timeout(() => {
      child.document.querySelector("#javascriptUndefinedLink").click();
      child.document.querySelector("iframe").onload = test.step_func_done(() => {
        assert_true(child.childLoaded, 'child.childLoaded');
      });
    }, 100);
  };
});
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/javascript-url-abort/javascript-url-abort-return-value-undefined.tentative.html"
}
```
