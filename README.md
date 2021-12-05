# md-img-to-table

Converts markdown image tags into html table tags

# Example

Input (`images.md`) :

```
![image.png](https://image.test/uploads/000000000000000000000000000000000000.png =WxH)
![image.png](https://image.test/uploads/000000000000000000000000000000000001.png =WxH)
![image.png](https://image.test/uploads/000000000000000000000000000000000002.png =WxH)
```

The command:

```
cat images.md | md-img-to-table
```

Output:

```
<table>
  <td>
    <img src="https://image.test/uploads/000000000000000000000000000000000000.png" />
  </td>
  <td>
    <img src="https://image.test/uploads/000000000000000000000000000000000001.png" />
  </td>
  <td>
    <img src="https://image.test/uploads/000000000000000000000000000000000002.png" />
  </td>
</table>
```
