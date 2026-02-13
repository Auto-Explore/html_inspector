# html/dom/reflection-original.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/reflection-original.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>HTML5 reflection tests</title>
<meta name=timeout content=long>
<p>This is <em>not</em> the authoritative conformance test suite for
reflection.  The authoritative tests can be found here, split up into sections:

<ul>
  <li><a href=reflection-metadata.html>Metadata elements</a>
  <li><a href=reflection-sections.html>Section elements</a>
  <li><a href=reflection-grouping.html>Grouping elements</a>
  <li><a href=reflection-text.html>Text elements</a>
  <li><a href=reflection-embedded.html>Embedded elements</a>
  <li><a href=reflection-tabular.html>Tabular elements</a>
  <li><a href=reflection-forms.html>Form elements</a>
  <li><a href=reflection-misc.html>Miscellaneous elements</a>
  <li><a href=reflection-obsolete.html>Obsolete elements</a>
</ul>

<p>This test suite is provided for implementers' convenience in debugging
failures.  It groups similar failures in a fashion that should help fix them.
It is not intended to be suitable for incorporation into automated testing
frameworks.

<p>Filter out errors matching a regex (operates on HTML not text, you have to manually escape entities): <input oninput="maskErrors(this.value)">

<p>Passed: <span id=passed>0</span> (<span id=percent></span>%).  Failed: <span id=failed>0</span>.  Time to complete: <span id=time>0</span> s.

<div id=errors></div>

<script src=original-harness.js></script>
<script src=elements-metadata.js></script>
<script src=elements-sections.js></script>
<script src=elements-grouping.js></script>
<script src=elements-text.js></script>
<script src=elements-embedded.js></script>
<script src=elements-tabular.js></script>
<script src=elements-forms.js></script>
<script src=elements-misc.js></script>
<script src=elements-obsolete.js></script>
<script src=reflection.js></script>
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
  "source_name": "html/dom/reflection-original.html"
}
```
