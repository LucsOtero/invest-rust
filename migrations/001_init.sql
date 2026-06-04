CREATE TABLE usuarios_app (
    id SERIAL PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    senha_hash TEXT NOT NULL
);

CREATE TABLE meus_ativos (
    id SERIAL PRIMARY KEY,
    id_usuario INT REFERENCES usuarios_app(id),
    codigo_ativo TEXT NOT NULL,
    qtd_cotas FLOAT NOT NULL,
    preco_compra FLOAT NOT NULL
);
