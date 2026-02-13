# html/semantics/scripting-1/the-script-element/import-attributes/invalid-type-attribute-error.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/import-attributes/invalid-type-attribute-error.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Handling of invalid module type import attributes</title>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
    setup({allow_uncaught_exception: true});

    window.log = [];

    window.addEventListener("error", ev => log.push(ev.error));

    const test_load = async_test(
        "Test that invalid module type attribute leads to TypeError on window.");
    window.addEventListener("load", test_load.step_func_done(ev => {
      assert_equals(log.length, 4);
      assert_equals(log[0].constructor, TypeError);
      assert_equals(log[1].constructor, TypeError);
      assert_equals(log[2].constructor, TypeError);
      assert_equals(log[3].constructor, TypeError);
    }));

    function unreachable() { log.push("unexpected"); }
</script>
<script type="module" src="./invalid-type-attribute.js" onerror="unreachable()"></script>
<script type="module" src="./empty-type-attribute.js" onerror="unreachable()"></script>
<script type="module" src="./js-type-attribute.js" onerror="unreachable()"></script>
<script type="module" src="./javascript-type-attribute.js" onerror="unreachable()"></script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/import-attributes/invalid-type-attribute-error.html"
}
```
