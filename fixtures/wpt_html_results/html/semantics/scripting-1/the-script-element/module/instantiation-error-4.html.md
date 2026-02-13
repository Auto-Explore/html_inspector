# html/semantics/scripting-1/the-script-element/module/instantiation-error-4.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/instantiation-error-4.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Handling of instantiation errors, 4</title>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
    setup({allow_uncaught_exception: true});

    window.log = [];
    const test_load = async_test(
        "Test that loading a graph in which a module is already " +
        "errored results in an error.");

    window.addEventListener("error", ev => {
      test_load.step(() => assert_equals(ev.error.constructor, SyntaxError));
      log.push(ev.message);
    });

    window.addEventListener("load", test_load.step_func_done(ev => {
      assert_equals(log.length, 4, 'Log length');
      assert_equals(log[1], 1);
      assert_equals(log[3], 2);
      assert_not_equals(log[0], log[2],
        'Instantiation error objects for different root scripts');
    }));

    function unreachable() { log.push("unexpected"); }
</script>
<script type="module" src="./instantiation-error-4a.js"
    onerror="unreachable()" onload="log.push(1)"></script>
<script type="module" src="./instantiation-error-4d.js"
    onerror="unreachable()" onload="log.push(2)"></script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/instantiation-error-4.html"
}
```
