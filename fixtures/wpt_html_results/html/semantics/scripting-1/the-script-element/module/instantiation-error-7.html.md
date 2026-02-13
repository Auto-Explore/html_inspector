# html/semantics/scripting-1/the-script-element/module/instantiation-error-7.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/instantiation-error-7.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Handling of instantiation errors, 7</title>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
    setup({allow_uncaught_exception: true});

    window.log = [];

    window.addEventListener("error", ev => log.push(ev.error));

    const test_load = async_test(
        "Test that ambiguous star exports lead to an instantiation error, " +
        "even when discovered through a star export, and that the correct " +
        "module is blamed.");
    // This is a variation of instantiation-error-6.html (see the explanation
    // there).
    window.addEventListener("load", test_load.step_func_done(ev => {
      const exn = log[0];
      assert_array_equals(log, [
          exn, 1,
          "instantiation-error-7d",
          "instantiation-error-7e",
          "instantiation-error-7c",
          "instantiation-error-7f",
          "instantiation-error-7b", 2
      ]);
      assert_equals(exn.constructor, SyntaxError);
    }));

    function unreachable() { log.push("unexpected"); }
</script>
<script type="module" src="./instantiation-error-7a.js"
    onerror="unreachable()" onload="log.push(1)"></script>
<script type="module" src="./instantiation-error-7b.js"
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/instantiation-error-7.html"
}
```
