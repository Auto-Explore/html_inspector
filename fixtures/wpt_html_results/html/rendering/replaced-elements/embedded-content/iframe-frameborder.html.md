# html/rendering/replaced-elements/embedded-content/iframe-frameborder.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/embedded-content/iframe-frameborder.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Presentational hint for iframe frameborder</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>

<div data-expect="0px">
 <iframe frameborder></iframe>
 <iframe frameborder=0></iframe>
 <iframe frameborder=-0></iframe>
 <iframe frameborder=0.5></iframe>
 <iframe frameborder=0.5e1></iframe>
 <iframe frameborder=no></iframe>
 <iframe frameborder=none></iframe>
 <iframe frameborder=error></iframe>
</div>

<div data-expect="2px">
 <iframe></iframe>
 <iframe frameborder=1></iframe>
 <iframe frameborder=-1></iframe>
 <iframe frameborder=10></iframe>
 <iframe frameborder=-10></iframe>
</div>

<script>
const iframes = document.querySelectorAll('iframe');
const borderWidthProps = ['border-top-width', 'border-right-width', 'border-bottom-width', 'border-left-width'];
for (const iframe of iframes) {
  test(() => {
    const computed = getComputedStyle(iframe);
    const expectedBorderWidth = iframe.parentNode.dataset.expect;
    for (const prop of borderWidthProps) {
      assert_equals(computed.getPropertyValue(prop), expectedBorderWidth, prop);
    }
    assert_equals(computed.getPropertyValue('border-style'), 'inset', 'border-style');
    assert_equals(computed.getPropertyValue('border-color'), getComputedStyle(document.documentElement).color, 'border-color');
  }, iframe.outerHTML);
}
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
  "source_name": "html/rendering/replaced-elements/embedded-content/iframe-frameborder.html"
}
```
