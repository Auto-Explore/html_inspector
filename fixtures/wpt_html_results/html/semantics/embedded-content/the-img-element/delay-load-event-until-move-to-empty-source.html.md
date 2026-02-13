# html/semantics/embedded-content/the-img-element/delay-load-event-until-move-to-empty-source.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/delay-load-event-until-move-to-empty-source.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Inline image element blocks load until source is changed to empty source</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<img src="/images/blue.png?pipe=trickle(d100)">
<script>

async_test(t => {
  const image = document.querySelector("img");

  assert_false(image.complete, "The image is loading initially");

  // Complete the test as soon as we obtained the window "load" event,
  // which should happen as soon as the image stops loading by moving
  // to an empty source.
  window.addEventListener("load", t.step_func_done(() => {
      assert_true(image.complete, "The image is no longer loading once the window 'load' event is dispatched");
  }));

  // Stop loading the image.
  image.src = "";
}, "Image element delays window's load event until the image changes to empty source");

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 280,
        "byte_start": 233,
        "col": 1,
        "line": 6
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
  "source_name": "html/semantics/embedded-content/the-img-element/delay-load-event-until-move-to-empty-source.html"
}
```
