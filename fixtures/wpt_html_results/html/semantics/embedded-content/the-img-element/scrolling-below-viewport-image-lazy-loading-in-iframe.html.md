# html/semantics/embedded-content/the-img-element/scrolling-below-viewport-image-lazy-loading-in-iframe.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/scrolling-below-viewport-image-lazy-loading-in-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Scrolling a lazy loaded image into view in an iframe</title>
<script src='/resources/testharness.js'></script>
<script src='/resources/testharnessreport.js'></script>
<iframe onload="async_test(this.contentWindow.run)" srcdoc="
<!DOCTYPE html>
<script src='/resources/testharness.js'></script>
<script src='/resources/testharnessreport.js'></script>
<script src='../resources/common.js'></script>

<h1>Scroll down...</h1>
<p>Scroll down...</p>
<p>Scroll down...</p>
<img id='below_iframe_viewport_img' src='resources/image.png' loading='lazy'
   onload='below_iframe_viewport_img.resolve();' onerror='below_iframe_viewport_img.reject();'>

<script>
  const scroll_trigger_img = new ElementLoadPromise('visible');
  const below_iframe_viewport_img = new ElementLoadPromise('below_iframe_viewport_img');

  function run(t) {
    below_iframe_viewport_img.element().scrollIntoView();
    below_iframe_viewport_img.promise
      .then(t.step_func(() => {
        assert_not_equals(below_iframe_viewport_img.element().width, 0, 'width should be greater than zero after scrolling');
        assert_not_equals(below_iframe_viewport_img.element().height, 0, 'height should be greater than zero after scrolling');
        t.done();
    }))
    .catch(t.unreached_func('The below_iframe_viewport image should load'));
  };
</script>
"></iframe>
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
  "source_name": "html/semantics/embedded-content/the-img-element/scrolling-below-viewport-image-lazy-loading-in-iframe.html"
}
```
