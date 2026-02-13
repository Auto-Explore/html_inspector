# html/semantics/embedded-content/the-img-element/Image-constructor.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/Image-constructor.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<title>DOM Image constructor Test</title>
<meta content="text/html; charset=UTF-8" http-equiv="Content-Type">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-img-element" />
<meta name="assert" content="Tests the Image constructor for the img-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>


<div id="log"></div>
<script>
  test(function() {
    var img = new Image();
    assert_true(img != undefined);
  }, "Image constructor works");

  test(function() {
    assert_equals(Image.prototype, HTMLImageElement.prototype);
  }, "Image and HTMLImageElement share a prototype");

  test(function() {
    assert_true((new Image()).localName === "img");
  }, "Image localName is img");

  test(function() {
    assert_true((new Image()).namespaceURI === "http://www.w3.org/1999/xhtml");
  }, "Image namespace URI is correct");

  test(function() {
      assert_equals(Image.name, "Image", "Image name should be Image (not HTMLImageElement)");
      assert_equals(Object.getPrototypeOf(Image), Function.prototype, "Image's prototype is Function.prototype");
      assert_equals(Image.prototype, HTMLImageElement.prototype, "Image.prototype is same as HTMLImageElement.prototype");
      assert_equals(Object.getPrototypeOf(new Image()), HTMLImageElement.prototype, "new Image()'s prototype is HTMLImageElement.prototype ");
      assert_equals(Object.getPrototypeOf(Image.prototype), HTMLElement.prototype, "Image.prototype's prototype is HTMLElement.prototype");

      const desc = Object.getOwnPropertyDescriptor(Image, "prototype");
      assert_false(desc.configurable, "Image.prototype is not configurable");
      assert_false(desc.enumerable, "Image.prototype is not enumerable");
      assert_false(desc.writable, "Image.prototype is not writable");
  }, "NamedConstructor creates the correct object structure.");

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
  "source_name": "html/semantics/embedded-content/the-img-element/Image-constructor.html"
}
```
