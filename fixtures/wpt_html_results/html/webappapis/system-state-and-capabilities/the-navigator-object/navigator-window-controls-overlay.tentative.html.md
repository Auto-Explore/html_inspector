# html/webappapis/system-state-and-capabilities/the-navigator-object/navigator-window-controls-overlay.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/system-state-and-capabilities/the-navigator-object/navigator-window-controls-overlay.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset='utf-8'>
<title>navigator.windowControlsOverlay</title>

<script src='/resources/testharness.js'></script>
<script src='/resources/testharnessreport.js'></script>

<script>
  test(function(){
    assert_idl_attribute(navigator, 'windowControlsOverlay');
  }, 'the windowControlsOverlay object should exist on the navigator object');

  test(function(){
    assert_idl_attribute(navigator.windowControlsOverlay, 'visible');
  }, 'visible should be a member of the windowControlsOverlay object');

  test(function(){
    assert_false(navigator.windowControlsOverlay.visible);
  }, 'visible should be false');

  test(function(){
    assert_idl_attribute(navigator.windowControlsOverlay, 'getTitlebarAreaRect');
  }, 'getTitlebarAreaRect should be a method of the windowControlsOverlay object');

  test(function(){
    var rect = navigator.windowControlsOverlay.getTitlebarAreaRect();
    assert_true(rect instanceof DOMRect);
  }, 'getTitlebarAreaRect return type should be DOMRect');

  test(function(){
    var rect = navigator.windowControlsOverlay.getTitlebarAreaRect();
    assert_equals(rect.x, 0);
    assert_equals(rect.y, 0);
    assert_equals(rect.width, 0);
    assert_equals(rect.height, 0);
  }, 'getTitlebarAreaRect should return a empty DOMRect');

  test(function(){
    assert_idl_attribute(navigator.windowControlsOverlay, 'ongeometrychange');
  }, 'ongeometrychange should be a member of the windowControlsOverlay object');
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
  "source_name": "html/webappapis/system-state-and-capabilities/the-navigator-object/navigator-window-controls-overlay.tentative.html"
}
```
