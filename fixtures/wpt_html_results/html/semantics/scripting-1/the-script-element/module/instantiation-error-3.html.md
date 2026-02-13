# html/semantics/scripting-1/the-script-element/module/instantiation-error-3.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/instantiation-error-3.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Handling of instantiation errors, 3</title>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
    setup({allow_uncaught_exception: true});

    const test_load = async_test(
        "Test that unresolvable cycles lead to SyntaxError events on window " +
        "and load events on script");

    window.log = [];
    window.addEventListener("error", ev => {
      test_load.step(() => assert_equals(ev.error.constructor, SyntaxError));
      log.push(ev.message);
    });

    window.addEventListener("load", test_load.step_func_done(ev => {
      assert_equals(log.length, 6, 'Log length');
      assert_equals(log[1], 1);
      assert_equals(log[3], 2);
      assert_equals(log[5], 3);
      assert_not_equals(log[0], log[2],
        'Instantiation error objects for different root scripts');
      assert_equals(log[0], log[4],
        'Instantiation error objects for the same root script');
    }));

    function unreachable() { log.push("unexpected"); }
</script>
<script type="module" src="./cycle-unresolvable.js"
    onerror="unreachable()" onload="log.push(1)" nomodule></script>
<script type="module" src="./cycle-unresolvable-a.js"
    onerror="unreachable()" onload="log.push(2)"></script>
<script type="module" src="./cycle-unresolvable.js"
    onerror="unreachable()" onload="log.push(3)"></script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.module.nomodule.disallowed",
      "message": "A “script” element with a “nomodule” attribute must not have a “type” attribute with the value “module”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1171,
        "byte_start": 1061,
        "col": 1,
        "line": 32
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/scripting-1/the-script-element/module/instantiation-error-3.html"
}
```
