# html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-stop-manual.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-stop-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: marquee-stop</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/multipage/obsolete.html#the-marquee-element">
<meta name="flags" content="interact">
<meta name="assert" content="Check the stop operation of HTMLMarqueeElement interface">
<h2>Steps:</h2>
<ol>
  <li>Click the 'Start' button to start the marquee element.</li>
</ol>
<h2>Expected result:</h2>
<ul>
  <li>The text "Test Marquee" stop moving when the 'Stop' button is clicked.</li>
</ul>
<input type="button" id="stop" value="Stop" />
<marquee id="test">Test Marquee</marquee>
<script>
  document.getElementById("stop").addEventListener("click", function(evt) {
    document.getElementById("test").stop();
  }, false);
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.marquee.obsolete",
      "message": "The “marquee” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 654,
        "byte_start": 635,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-stop-manual.html"
}
```
