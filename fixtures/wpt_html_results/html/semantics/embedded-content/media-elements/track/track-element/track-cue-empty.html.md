# html/semantics/embedded-content/media-elements/track/track-element/track-cue-empty.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-empty.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Invoke getCueAsHTML() on an empty cue</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
test(function() {
    var emptyCue = new VTTCue(0, 0, "");
    var fragment = emptyCue.getCueAsHTML();

    // The getCueAsHTML() method should return a document fragment.
    assert_true(fragment instanceof DocumentFragment);

    // The document fragment should have one child, an empty Text node.
    assert_equals(fragment.childNodes.length, 1);
    assert_equals(fragment.childNodes[0].constructor.name, Text.name);
    assert_equals(fragment.childNodes[0].length, 0);
    assert_equals(fragment.childNodes[0].data, "");
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-empty.html"
}
```
