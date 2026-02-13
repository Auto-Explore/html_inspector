# html/semantics/scripting-1/the-script-element/import-attributes/invalid-import-errors-order.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/import-attributes/invalid-import-errors-order.html",
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

    var log = [];
    window.addEventListener("error", ev => log.push(ev.error));

    const test_load = async_test(
        "Test that the errors order for invalid import declarations is" +
        " specifier, then attribute key, and then type attribute value.");
    window.addEventListener("load", test_load.step_func_done(ev => {
      assert_equals(log.length, 3);
      assert_equals(log[0].constructor, SyntaxError);
      assert_equals(log[1].constructor, SyntaxError);
      assert_equals(log[2].constructor, SyntaxError);
    }));

    function unreachable() { log.push("unexpected"); }
</script>
<script type="module" onerror="unreachable()">
  // unknown attribute is reported before invalid specifier
  import "INVALID" with { unknown: "foo" };
</script>
<script type="module" onerror="unreachable()">
  // unknown attribute is reported before unknown type
  import "./valid" with { unknown: "foo", type: "unknown" };
</script>
<script type="module" onerror="unreachable()">
  // unknown attribute is reported before invalid specifier in subsequent import
  import "./valid" with { unknown: "foo" };
  import "INVALID";
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
  "source_name": "html/semantics/scripting-1/the-script-element/import-attributes/invalid-import-errors-order.html"
}
```
