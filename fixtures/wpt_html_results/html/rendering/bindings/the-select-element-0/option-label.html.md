# html/rendering/bindings/the-select-element-0/option-label.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/bindings/the-select-element-0/option-label.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Option labels</title>
<link rel="match" href="option-label-ref.html">
<select size=12></select>
<script>
var select = document.getElementsByTagName("select")[0], option;

option = document.createElement("option");
select.appendChild(option);

option = document.createElement("option");
option.setAttribute("label", "")
select.appendChild(option);

option = document.createElement("option");
option.setAttribute("label", "label")
select.appendChild(option);

option = document.createElement("option");
option.setAttributeNS("http://www.example.com/", "label", "label")
select.appendChild(option);

option = document.createElement("option");
option.appendChild(document.createTextNode(" child "));
select.appendChild(option);

option = document.createElement("option");
option.appendChild(document.createTextNode(" child "));
option.setAttribute("label", "")
select.appendChild(option);

option = document.createElement("option");
option.appendChild(document.createTextNode(" child "));
option.setAttribute("label", "label")
select.appendChild(option);

option = document.createElement("option");
option.appendChild(document.createTextNode(" child "));
option.setAttributeNS("http://www.example.com/", "label", "label")
select.appendChild(option);


option = document.createElement("option");
option.appendChild(document.createTextNode(" child "));
option.appendChild(document.createTextNode(" node "));
select.appendChild(option);

option = document.createElement("option");
option.appendChild(document.createTextNode(" child "));
option.appendChild(document.createTextNode(" node "));
option.setAttribute("label", "")
select.appendChild(option);


option = document.createElement("option");
option.appendChild(document.createTextNode(" child "));
option.appendChild(document.createTextNode(" node "));
option.setAttribute("label", "label")
select.appendChild(option);

option = document.createElement("option");
option.appendChild(document.createTextNode(" child "));
option.appendChild(document.createTextNode(" node "));
option.setAttributeNS("http://www.example.com/", "label", "label")
select.appendChild(option);
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
  "source_name": "html/rendering/bindings/the-select-element-0/option-label.html"
}
```
