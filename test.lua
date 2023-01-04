local bufnr = 6
local parm =  {
  textDocument = vim.lsp.util.make_text_document_params(bufnr),
  position = nil, -- get em all
}
local method = "textDocument/codeLens"
local handler = function (err, result, ctx)
  print(vim.inspect(result))
end
print("done")
vim.lsp.buf_request(bufnr, method, parm, handler)
