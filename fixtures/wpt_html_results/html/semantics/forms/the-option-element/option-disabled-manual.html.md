# html/semantics/forms/the-option-element/option-disabled-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-option-element/option-disabled-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTMLOptionElement Test: disabled</title>
<meta name="flags" content="interact">
<link rel="author" title="Intel" href="http://www.intel.com/">

<div>
  <select>
    <option id="testOption1" text="Option1" >Option1</option>
    <option id="testOption2" disabled >Option2</option>
    <option id="testOption3" >Option3</option>
  </select>
</div>

<h2>Description</h2>
<p>
  This test validates that an option element is disabled if its disabled attribute is present.
</p>

<h2>Test steps:</h2>
<ol>
  <li>
    Click the select flag to select 'Option2'
  </li>
</ol>

<h2>Result:</h2>
<p>Test passes if not able to select 'Option2'</p>
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
  "source_name": "html/semantics/forms/the-option-element/option-disabled-manual.html"
}
```
