# html/semantics/scripting-1/the-script-element/module/instantiation-error-6.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/instantiation-error-6.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Handling of instantiation errors, 6</title>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
    setup({allow_uncaught_exception: true});

    window.log = [];

    window.addEventListener("error", ev => log.push(ev.error));

    const test_load = async_test(
        "Test that ambiguous star exports lead to an instantiation error " +
        "and that the correct module is blamed.");
    // Concretely, instantiation-error-6a.js fails to instantiate because it
    // requests a name from instantion-error-6b.js that is ambiguous there.
    // instantiation-error-6b.js itself, however, is fine, and it instantiates
    // and evaluates successfully.
    window.addEventListener("load", test_load.step_func_done(ev => {
      const exn = log[0];
      assert_array_equals(log, [
          exn, 1,
          "instantiation-error-6c",
          "instantiation-error-6d",
          "instantiation-error-6b", 2
      ]);
      assert_equals(exn.constructor, SyntaxError);
    }));

    function unreachable() { log.push("unexpected"); }
</script>
<script type="module" src="./instantiation-error-6a.js"
    onerror="unreachable()" onload="log.push(1)"></script>
<script type="module" src="./instantiation-error-6b.js"
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/instantiation-error-6.html"
}
```
