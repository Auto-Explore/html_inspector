# html/semantics/scripting-1/the-script-element/module/compilation-error-2.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/compilation-error-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Handling of compilation errors, 2</title>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
    setup({allow_uncaught_exception: true});

    window.log = [];

    window.addEventListener("error", ev => log.push(ev.error));

    const test_load = async_test(
        "Test that syntax errors lead to SyntaxError events on window, " +
        "and that exceptions are remembered.");
    window.addEventListener("load", test_load.step_func_done(ev => {
      assert_equals(log.length, 5);
      assert_equals(log[0].constructor, SyntaxError);
      assert_true(log.every(exn => exn === log[0]));
    }));

    function unreachable() { log.push("unexpected"); }
</script>
<script type="module" src="./syntaxerror-nested.js" onerror="unreachable()"></script>
<script type="module" src="./syntaxerror-nested.js" onerror="unreachable()"></script>
<script type="module" src="./syntaxerror.js" onerror="unreachable()"></script>
<script type="module" src="./syntaxerror-nested.js" onerror="unreachable()"></script>
<script type="module" src="./syntaxerror.js" onerror="unreachable()"></script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/compilation-error-2.html"
}
```
