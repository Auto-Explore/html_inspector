# html/semantics/scripting-1/the-script-element/module/evaluation-error-2.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/evaluation-error-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Handling of evaluation errors, 2</title>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
    setup({allow_uncaught_exception: true});

    window.log = [];

    window.addEventListener("error", ev => log.push(ev.error));
    window.addEventListener("unhandledrejection", unreachable);

    const test_load = async_test(
        "Test that ill-founded cyclic dependencies cause ReferenceError " +
        "during evaluation, which leads to error events on window, and that " +
        "exceptions are remembered.\n");
    window.addEventListener("load", test_load.step_func_done(ev => {
      const exn = log[1];
      assert_array_equals(log,
          ["cycle-tdz-access-a", exn, "load", exn, "load", exn, "load"]);
      assert_equals(exn.constructor, ReferenceError);
    }));

    function logLoad() { log.push("load"); }

    function unreachable() { log.push("unexpected"); }
</script>
<script type="module" src="cycle-tdz-access.js" async onerror="unreachable()"
    onload="logLoad()"></script>
<script type="module" src="cycle-tdz-access.js" nomodule onerror="unreachable()"
    onload="logLoad()"></script>
<script type="module" src="cycle-tdz-access.js" onerror="unreachable()"
    onload="logLoad()"></script>
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
        "byte_end": 1198,
        "byte_start": 1094,
        "col": 1,
        "line": 31
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/evaluation-error-2.html"
}
```
