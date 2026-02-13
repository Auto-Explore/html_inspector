# html/semantics/embedded-content/the-img-element/image-loading-lazy-in-scroller-far.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy-in-scroller-far.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://html.spec.whatwg.org/#lazy-load-root-margin">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<style>
  #scroller {
    width: 100px;
    height: 100px;
    overflow: scroll;
    background-color: gray;
  }

  #spacer {
    width: 130px;
    height: 10000vh;
  }

  #target {
    width: 100px;
    height: 100px;
  }
</style>

<div id="scroller">
  <div id="spacer"></div>
  <img
    id="target"
    src="resources/green.png"
    loading="lazy"
    onload="img_onload();"
  >
</div>

<script>
  const t = async_test(
    "Test that lazy-loaded images do not load when far from viewport."
  );

  function img_onload() {
    t.unreached_func(
      "Lazy-loading image far from viewport should not load."
    )();
  }

  t.step_timeout(() => {
    t.done();
  }, 2000);
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
        "byte_end": 581,
        "byte_start": 481,
        "col": 3,
        "line": 27
      }
    },
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy-in-scroller-far.html"
}
```
