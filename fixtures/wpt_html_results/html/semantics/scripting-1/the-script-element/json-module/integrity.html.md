# html/semantics/scripting-1/the-script-element/json-module/integrity.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/json-module/integrity.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>&lt;script> integrity=""</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#prepare-a-script">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
window.matchesLog = [];
window.matchesEvents = [];

window.mismatchesLog = [];
window.mismatchesEvents = [];
</script>
<script type="module" src="integrity-matches.js" integrity="sha384-kc1K2KFKQhnYE1AdnpmUUpFVnxz1GCgGbQ19e3zmXrZw23rgpwa9il4/pHp7NYWA"  onload="window.matchesEvents.push('load');" onerror="window.matchesEvents.push('error')"></script>
<script type="module" src="integrity-mismatches.js" integrity="sha384-doesnotmatch" onload="window.mismatchesEvents.push('load');" onerror="window.mismatchesEvents.push('error')"></script>

<script type="module">
test(() => {
  assert_array_equals(window.matchesLog, ["integrity-matches,json:42"], "The module and its dependency must have executed");
  assert_array_equals(window.matchesEvents, ["load"], "The load event must have fired");
}, "The integrity attribute must be verified on the top-level of a module loading a JSON module and allow it to execute when it matches");

test(() => {
  assert_array_equals(window.mismatchesLog, [], "The module and its dependency must not have executed");
  assert_array_equals(window.mismatchesEvents, ["error"], "The error event must have fired");
}, "The integrity attribute must be verified on the top-level of a module loading a JSON module and not allow it to execute when there's a mismatch");
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
  "source_name": "html/semantics/scripting-1/the-script-element/json-module/integrity.html"
}
```
