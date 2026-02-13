# html/scripting/the-script-element/classic-external-unknown-scheme.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/scripting/the-script-element/classic-external-unknown-scheme.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="UTF-8">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
window.onload = function() {
    done();
}
setup({ single_test: true });
var flag = false;
function unknown() {
    document.write("<scr" + "ipt>assert_false(flag); flag = true; assert_equals(document.readyState, 'loading');</scr" + "ipt>");
}
function known() {
    document.write("<scr" + "ipt>assert_true(flag); assert_equals(document.readyState, 'loading');</scr" + "ipt>");
}
</script>
<script onerror="unknown();" src="unknown://example/"></script>
<script onerror="known();" src="resources/must-not-exist.js"></script>
<script>
assert_equals(document.getElementsByTagName("script").length, 8);
</script>
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
  "source_name": "html/scripting/the-script-element/classic-external-unknown-scheme.html"
}
```
