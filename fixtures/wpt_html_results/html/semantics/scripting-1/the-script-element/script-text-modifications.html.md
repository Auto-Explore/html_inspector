# html/semantics/scripting-1/the-script-element/script-text-modifications.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/script-text-modifications.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<head>
<meta charset=utf-8>
<title>Modify HTMLScriptElement's text after #prepare-a-script</title>
<link rel=help href="https://html.spec.whatwg.org/multipage/scripting.html#prepare-a-script">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
var t = async_test("Modify inline script element's text " +
                   "after prepare-a-script before evaluation");

function changeScriptText() {
  document.querySelector('#script0').textContent =
    't.unreached_func("This should not be evaluated")();';
}

t.step_timeout(changeScriptText, 500);
</script>

<!-- This is "a style sheet that is blocking scripts" and thus ... -->
<link rel="stylesheet" href="/common/slow.py?pipe=trickle(d1)"></link>

<!-- This inline script becomes a parser-blocking script, and thus
the step_timeout is evaluated after script0 is inserted into DOM,
prepare-a-script'ed, but before its evaluation. -->
<script id="script0">
t.step(() => {
    // When this is evaluated after the stylesheet is loaded,
    // script0's textContent is modified by the async script above,
    // but the evaluated script is still the original script here,
    // not what is overwritten, because "child text content" is taken in
    // #prepare-a-script and passed to "creating a classic script".
    var s = document.getElementById('script0');
    assert_equals(s.textContent,
                  't.unreached_func("This should not be evaluated")();',
                  "<script>'s textContent should be already modified");
    t.done();
  });
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “link”.",
      "severity": "Error",
      "span": {
        "byte_end": 784,
        "byte_start": 777,
        "col": 64,
        "line": 22
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
  "source_name": "html/semantics/scripting-1/the-script-element/script-text-modifications.html"
}
```
