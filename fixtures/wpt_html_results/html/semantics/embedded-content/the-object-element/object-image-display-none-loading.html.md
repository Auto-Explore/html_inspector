# html/semantics/embedded-content/the-object-element/object-image-display-none-loading.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-object-element/object-image-display-none-loading.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://crbug.com/41477900">
<link rel="help" href="https://html.spec.whatwg.org/multipage/iframe-embed-object.html#the-object-element">
<!--
  Based on step 2 of the spec algorithm:
  "... if the element is not being rendered,
  then jump to the step below labeled fallback."
-->

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
  const t = async_test(
    "Test that object image not displayed is not loaded unnecessarily."
  );

  function obj_onload() {
    t.unreached_func(
      "Object image not displayed on screen should not load."
    )();
  }

  t.step_timeout(() => {
    t.done();
  }, 2000);
</script>

<style>
  #target {
    display: none;
  }
</style>

<object
  data="/images/red.png"
  id="target"
  onload="obj_onload();"
  type="image/png"
>
  Fallback Text
</object>
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
  "source_name": "html/semantics/embedded-content/the-object-element/object-image-display-none-loading.html"
}
```
