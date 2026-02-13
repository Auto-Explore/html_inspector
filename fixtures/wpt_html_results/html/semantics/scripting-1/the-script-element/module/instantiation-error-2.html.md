# html/semantics/scripting-1/the-script-element/module/instantiation-error-2.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/instantiation-error-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Handling of instantiation errors, 2</title>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
    setup({allow_uncaught_exception: true});

    const test_load = async_test(
        "Test that missing exports lead to SyntaxError events on window and " +
        "load events on script");

    window.log = [];
    window.addEventListener("error", ev => {
      test_load.step(() => assert_equals(ev.error.constructor, SyntaxError));
      log.push(ev.message);
    });

    window.addEventListener("load", test_load.step_func_done(ev => {
      const msg = log[0];
      assert_array_equals(log, [msg, 1, msg, 2, msg, 3, msg, 4, msg, 5]);
    }));

    function unreachable() { log.push("unexpected"); }
</script>
<script type="module" src="./missing-export-nested.js"
    onerror="unreachable()" onload="log.push(1)"></script>
<script type="module" src="./missing-export-nested.js"
    onerror="unreachable()" onload="log.push(2)"></script>
<script type="module" src="./missing-export.js"
    onerror="unreachable()" onload="log.push(3)"></script>
<script type="module" src="./missing-export-nested.js"
    onerror="unreachable()" onload="log.push(4)"></script>
<script type="module" src="./missing-export.js"
    onerror="unreachable()" onload="log.push(5)"></script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/instantiation-error-2.html"
}
```
