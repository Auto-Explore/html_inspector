# html/semantics/scripting-1/the-script-element/module/integrity.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/integrity.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>&lt;script> integrity=""</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#prepare-a-script">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
window.inlineRan = false;

window.matchesLog = [];
window.matchesEvents = [];

window.mismatchesLog = [];
window.mismatchesEvents = [];
</script>

<script type="module" integrity="sha384-garbage">
window.inlineRan = true;
</script>

<script type="module" src="integrity-matches.js" integrity="sha384-1/XwTy38IAlmvk1O674Efus1/REqfuX6x0V/B2/GX5R3lNbRjhrIwlWyEDPyOwpN" onload="window.matchesEvents.push('load');" onerror="window.matchesEvents.push('error')"></script>
<script type="module" src="integrity-mismatches.js" integrity="sha384-doesnotmatch" onload="window.mismatchesEvents.push('load');" onerror="window.mismatchesEvents.push('error')"></script>

<script type="module">
test(() => {
  assert_true(window.inlineRan);
}, "The integrity attribute must have no affect on inline module scripts");

test(() => {
  assert_array_equals(window.matchesLog, ["integrity-matches-inner", "integrity-matches"], "The module and its dependency must have executed");
  assert_array_equals(window.matchesEvents, ["load"], "The load event must have fired");
}, "The integrity attribute must be verified on the top-level of a module and allow it to execute when it matches");

test(() => {
  assert_array_equals(window.mismatchesLog, [], "The module and its dependency must not have executed");
  assert_array_equals(window.mismatchesEvents, ["error"], "The error event must have fired");
}, "The integrity attribute must be verified on the top-level of a module and not allow it to execute when there's a mismatch");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.module.inline.integrity.disallowed",
      "message": "An inline “script” element with “type=module” must not have an “integrity” attribute.",
      "severity": "Warning",
      "span": {
        "byte_end": 545,
        "byte_start": 496,
        "col": 1,
        "line": 19
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/integrity.html"
}
```
