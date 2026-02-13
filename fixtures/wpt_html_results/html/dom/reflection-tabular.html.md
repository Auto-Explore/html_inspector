# html/dom/reflection-tabular.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/reflection-tabular.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>HTML5 reflection tests: tabular elements</title>
<meta name=timeout content=long>
<p>Implementers looking to fix bugs might want to use the <a
href=reflection-original.html>original version</a> of this suite's test
framework, which conveniently aggregates similar errors and only reports
failures.  This file is (part of) the authoritative conformance test suite, and
is suitable for incorporation into automated test suites.

<div id=log></div>

<script src="/resources/testharness.js"></script>
<script src=/resources/testharnessreport.js></script>
<script src=original-harness.js></script>
<script src=new-harness.js></script>
<script src=elements-tabular.js></script>
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
  "source_name": "html/dom/reflection-tabular.html"
}
```
