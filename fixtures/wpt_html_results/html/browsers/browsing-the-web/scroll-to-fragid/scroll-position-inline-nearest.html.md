# html/browsers/browsing-the-web/scroll-to-fragid/scroll-position-inline-nearest.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/scroll-to-fragid/scroll-position-inline-nearest.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html style="writing-mode: vertical-lr;">
<head>
<meta charset="UTF-8">
<title>Fragment Navigation: inline start position should not scroll out of content range</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#scroll-to-the-fragment-identifier">
<link rel="author" href="mailto:mrobinson@igalia.com" title="Martin Robinson">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
    <!-- When scrolling to this fragment the viewport inline position should not
        change because, it is already fully enclosed by the viewport and page width. -->
    <div id="test1" style="position: absolute; top: 5000px; left: 100px; height: 100px; width: 100px;"></div>
<script>

var t = async_test("ScrollToFragment");
t.step(() => {
    location.hash = "test1";
    setTimeout(t.step_func(() => {
        assert_true(window.scrollY > 0);
        assert_true(window.scrollY < 5000);
        assert_equals(window.scrollX, 0);
        t.done();
    }));
});

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
  "source_name": "html/browsers/browsing-the-web/scroll-to-fragid/scroll-position-inline-nearest.html"
}
```
