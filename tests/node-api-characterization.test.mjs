import test from 'node:test';
import assert from 'node:assert/strict';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';

const require = createRequire(import.meta.url);
const sapling = require(fileURLToPath(new URL('../packages/sapling-wasm/dist/index.node.js', import.meta.url)));

const seed0123 = Buffer.from([...Array(32).keys()]);

function reverseHexBytes(hex) {
  const bytes = Buffer.from(hex, 'hex');
  return Buffer.from(bytes.reverse());
}

function hexBytes(hex) {
  return Buffer.from(hex, 'hex');
}

function octezCommitments() {
  return [
    '556f3af94225d46b1ef652abc9005dee873b2e245eef07fd5be587e0f21023b0',
    '5814b127a6c6b8f07ed03f0f6e2843ff04c9851ff824a4e5b4dad5b5f3475722',
    '6c030e6d7460f91668cc842ceb78cdb54470469e78cd59cf903d3a6e1aa03e7c',
    '30a0d08406b9e3693ee4c062bd1e6816f95bf14f5a13aafa1d57942c6c1d4250',
    '12fc3e7298eb327a88abcc406fbe595e45dddd9b4209803b2e0baa3a8663ecaa',
    '021a35cfe13d16891c1409d0f6e8865f51dd54792e5108a6f9e55e0dd44867f7',
    '2e0bfc1e123edcb6252251611650f3667371f781b60302385c414716c75e8abc',
    '11a5e54bf9a9b57e1c163904999ad1527f1e126c685111e18193decca2dd1ada',
    '4674f7836089063143fc18b673b2d92f888c63380e3680385d47bcdbd5fe273a',
    '0830165f36a69e416d51cc09cc5668692dee35d98539d3317999fdf87d8fcac7',
    '02372c746664e0898576972ca6d0500c7c8ec42f144622349d133b06e837faf0',
    '08c6d7dd3d2e387f7b84d6769f2b6cbe308918ab81e0f7321bd0945868d7d4e6',
    '26e8c4061f2ad984d19f2c0a4436b9800e529069c0b0d3186d4683e83bb7eb8c',
    '037cc2391338956026521beca5c81b541b7f2d1ead7758bf4d1588dbbcb8fa22',
    '1cc467cfd2b504e156c9a38bc5c0e4f5ea6cc208054d2d0653a7e561ac3a3ef4',
    '15ac4057a9a94536eca9802de65e985319e89627c9c64bc94626b712bc61363a'
  ].map(reverseHexBytes);
}

function octezExpectedRoots() {
  return [
    '8c3daa300c9710bf24d2595536e7c80ff8d147faca726636d28e8683a0c27703',
    '8611f17378eb55e8c3c3f0a5f002e2b0a7ca39442fc928322b8072d1079c213d',
    '3db73b998d536be0e1c2ec124df8e0f383ae7b602968ff6a5276ca0695023c46',
    '7ac2e6442fec5970e116dfa4f2ee606f395366cafb1fa7dfd6c3de3ce18c4363',
    '6a8f11ab2a11c262e39ed4ea3825ae6c94739ccf94479cb69402c5722b034532',
    '149595eed0b54a7e694cc8a68372525b9ae2c7b102514f527460db91eb690565',
    '8c0432f1994a2381a7a4b5fda770336011f9e0b30784f9a5597901619c797045',
    'e780c48d70420601f3313ff8488d7766b70c059c53aa3cda2ff1ef57ff62383c',
    'f919f03caaed8a2c60f58c0d43838f83e670dc7e8ccd25daa04a13f3e8f45541',
    '74f32b36629724038e71cbd6823b5a666440205a7d1a9242e95870b53d81f34a',
    'a4af205a4e1ee02102866b23a68930ac33efda9235832f49b17fcc4939be4525',
    'a946a42f1636045a16e65b2308e036d9da70089686c87c692e45912bd1cab772',
    'a1db2dbac055364c1cb43cbeb49c7e2815bff855122602a2ad0fb981a91e0e39',
    '16329b3ba4f0640f4d306532d9ea6ba0fbf0e70e44ed57d27b4277ed9cda6849',
    '7b6523b2d9b23f72fec6234aa6a1f8fae3dba1c6a266023ea8b1826feba7a25c',
    '5c0bea7e17bde5bee4eb795c2eec3d389a68da587b36dd687b134826ecc09308'
  ].map(hexBytes);
}

async function octezUncommittedNodes(depth) {
  const nodes = [hexBytes('0100000000000000000000000000000000000000000000000000000000000000')];

  for (let height = 0; height < depth; height++) {
    nodes.push(await sapling.merkleHash(height, nodes[height], nodes[height]));
  }

  return nodes;
}

async function octezRootFromLeaves(depth, leaves) {
  const defaults = await octezUncommittedNodes(depth);
  let levelNodes = Array.from({ length: 1 << depth }, () => Buffer.from(defaults[0]));

  for (let index = 0; index < leaves.length; index++) {
    levelNodes[index] = leaves[index];
  }

  for (let height = 0; height < depth; height++) {
    const nextLevel = [];

    for (let index = 0; index < levelNodes.length; index += 2) {
      nextLevel.push(await sapling.merkleHash(height, levelNodes[index], levelNodes[index + 1]));
    }

    levelNodes = nextLevel;
  }

  return levelNodes[0];
}

const octezVectorCases = [
  {
    ivk: 'b70b7cd0ed03cbdfd7ada9502ee245b13e569d54a5719d2daa0f5f1451479204',
    diversifier: 'f19d9b797e39f337445839',
    pkd: 'db4cd2b0aac4f7eb8ca131f16567c445a9555126d3c29f14e3d776e841ae7415',
    value: '0',
    rcm: '39176dac39ace4980ecc8d778e89860255ec3615060000000000000000000000',
    cmu: 'cb3cf9153270d57eb914c6c2bcc01850c9fed44fce0806278f083ef2dd076439'
  },
  {
    ivk: 'c518384466b26988b5109067418d192d9d6bd0d9232205d77418c240fc68a406',
    diversifier: 'aef180f6e34e354b888f81',
    pkd: 'a6b13ea336ddb7a67bb09a0e68e9d3cfb39210831ea3a296ba09a922060fd38b',
    value: '395043257984320',
    rcm: '478ba0ee6e1a75b600036f26f18b7015ab556beddf8b960238869f89dd804e06',
    cmu: '9255db54569746f13391c68ef872d8a84e33882fbf17de7da64b142f0b9d2069'
  }
];

test('built Node package preserves key derivation outputs', async () => {
  // Authoritative source:
  // https://raw.githubusercontent.com/zcash/zcash-test-vectors/master/test-vectors/json/sapling_zip32.json
  const path = 'm/1/2h';
  const xsk = await sapling.getExtendedSpendingKey(seed0123, path);
  const xfvk = await sapling.getExtendedFullViewingKey(seed0123, path);
  const xfvkFromXsk = await sapling.getExtendedFullViewingKeyFromSpendingKey(xsk);
  const ovk = await sapling.getOutgoingViewingKey(xfvk);
  const ivk = await sapling.getIncomingViewingKey(xfvk);

  assert.equal(
    xsk.toString('hex'),
    '02db999e070200008097ce15f4ed1b9739b0262a463bcb3dc9b3bd2323a9baa441ca42777383a8d4358be8113cee3413a71f82c41fc8da517be134049832e6825c92da6b84fee4c60d3778059dc569e7d0d32391573f951bbde92fc6b9cf614773661c5c273aa6990ccf81182e96223c028ce3d6eb4794d3113b95069d14c57588e193b65efc2813bca3eda19f9eff46ca12dfa1bf10371b48d1b4a40c4d05a0d8dce0e7dc62b07b37'
  );
  assert.equal(
    xfvk.toString('hex'),
    '02db999e070200008097ce15f4ed1b9739b0262a463bcb3dc9b3bd2323a9baa441ca42777383a8d435a6c5925a0f85fa4f1e405e3a4970d0c4a4b4814438f4e9d4520e20f7fdcf3841304e305916216beb7b654d8aae50ecd188fcb384bc36c00c664f307725e2ee11cf81182e96223c028ce3d6eb4794d3113b95069d14c57588e193b65efc2813bca3eda19f9eff46ca12dfa1bf10371b48d1b4a40c4d05a0d8dce0e7dc62b07b37'
  );
  assert.equal(xfvkFromXsk.toString('hex'), xfvk.toString('hex'));
  assert.equal(ovk.toString('hex'), 'cf81182e96223c028ce3d6eb4794d3113b95069d14c57588e193b65efc2813bc');
  assert.equal(ivk.toString('hex'), 'a2a13c1e38b45984445803e430a683c90bb2e14d4c8692ff253a6484dd9bb504');
});

test('built Node package preserves address derivation outputs', async () => {
  // Authoritative source:
  // https://raw.githubusercontent.com/zcash/zcash-test-vectors/master/test-vectors/json/sapling_zip32.json
  const xfvk = await sapling.getExtendedFullViewingKey(seed0123, 'm/1/2h');
  const address = await sapling.getPaymentAddressFromViewingKey(xfvk);
  const nextAddress = await sapling.getNextPaymentAddressFromViewingKey(xfvk, address.index);
  const diversifier = await sapling.getDiversifiedFromRawPaymentAddress(address.raw);
  const pkd = await sapling.getPkdFromRawPaymentAddress(address.raw);
  const rawFromIvk = await sapling.getRawPaymentAddressFromIncomingViewingKey(
    await sapling.getIncomingViewingKey(xfvk),
    diversifier
  );

  assert.equal(address.index.toString('hex'), '0000000000000000000000');
  assert.equal(address.raw.toString('hex'), 'e8d03793cdd2bacc9c7041ad5ec1877b8ca3ada20125535e840498712bda116dbc506edaf52d94fb8c72de');
  assert.equal(nextAddress.index.toString('hex'), '0100000000000000000000');
  assert.equal(nextAddress.raw.toString('hex'), '020a7a6b0bf84d3e899f68376c89212336c7a6e7fa7552326126805d98310a9cbbb3cd030ccf4412de3b89');
  assert.equal(diversifier.toString('hex'), 'e8d03793cdd2bacc9c7041');
  assert.equal(pkd.toString('hex'), 'ad5ec1877b8ca3ada20125535e840498712bda116dbc506edaf52d94fb8c72de');
  assert.equal(rawFromIvk.toString('hex'), address.raw.toString('hex'));
});

test('built Node package preserves alternate path viewing-key and address outputs', async () => {
  // Authoritative source:
  // https://raw.githubusercontent.com/zcash/zcash-test-vectors/master/test-vectors/json/sapling_zip32.json
  const xfvk = await sapling.getExtendedFullViewingKey(seed0123, 'm/1/2h/3');
  const ovk = await sapling.getOutgoingViewingKey(xfvk);
  const ivk = await sapling.getIncomingViewingKey(xfvk);
  const address = await sapling.getPaymentAddressFromViewingKey(xfvk);

  assert.equal(
    xfvk.toString('hex'),
    '0348c18375030000008d937bcf81ba430d5b49afc0a403367b1fd99879ecba41be051c5a4aa7d6e7e8b185c57b509c2536c4f2d326d766c8fab25447de5375a9328d649ddabd97a6a3db88049e02d207568afc42e07db2abed500b2701c01bbff36399764b81c0664f69b9e0fa1c4b3deb91d53beee871156121474b8b62ef24134478dc3499691af6becb50c363bb2ed9da5c3043ceb0f1a0527bf836b29a35f7c0c9f261123be56e'
  );
  assert.equal(ovk.toString('hex'), '69b9e0fa1c4b3deb91d53beee871156121474b8b62ef24134478dc3499691af6');
  assert.equal(ivk.toString('hex'), 'b0a5f337232f2c3dac70c2a410fa561fc45d8cc59cda246d31c8b1715a57d900');
  assert.equal(address.index.toString('hex'), '0100000000000000000000');
  assert.equal(address.raw.toString('hex'), '030ffb263a939e230e96dd0805ba6dbe98d91f30f3b1ac40a8bca48ce1304da1da1012f81415dd7061c5f1');
});

test('built Node package preserves Sapling merkle hashing outputs', async () => {
  // Authoritative source:
  // https://raw.githubusercontent.com/zcash/zcash-test-vectors/master/zcash_test_vectors/sapling/merkle_tree.py
  const lhs = reverseHexBytes('87a086ae7d2252d58729b30263fb7b66308bf94ef59a76c9c86e7ea016536505');
  const rhs = reverseHexBytes('a75b84a125b2353da7e8d96ee2a15efe4de23df9601b9d9564ba59de57130406');
  const expected = reverseHexBytes('5bf43b5736c19b714d1f462c9d22ba3492c36e3d9bbd7ca24d94b440550aa561');

  assert.deepEqual(await sapling.merkleHash(25, lhs, rhs), expected);
});

test('built Node package preserves Octez empty-tree root', async () => {
  // Authoritative source:
  // repos/tezos/src/lib_sapling/test/test_merkle.ml
  let root = (await octezUncommittedNodes(32))[0];

  for (let height = 0; height < 32; height++) {
    root = await sapling.merkleHash(height, root, root);
  }

  assert.deepEqual(
    root,
    reverseHexBytes('3e49b5f954aa9d3545bc6c37744661eea48d7c34e3000d82b7f0010c30f4c2fb')
  );
});

test('built Node package preserves Octez incremental merkle roots', async () => {
  // Authoritative source:
  // repos/tezos/src/lib_sapling/test/test_merkle.ml
  const commitments = octezCommitments();
  const expectedRoots = octezExpectedRoots();

  for (let index = 0; index < expectedRoots.length; index++) {
    const actualRoot = await octezRootFromLeaves(4, commitments.slice(0, index + 1));
    assert.deepEqual(actualRoot, expectedRoots[index]);
  }
});

test('built Node package preserves Octez payment-address and commitment vectors', async () => {
  // Authoritative source:
  // repos/tezos/src/lib_sapling/test/vectors.csv
  // That file documents its upstream lineage as:
  // https://github.com/zcash-hackworks/zcash-test-vectors
  for (const vector of octezVectorCases) {
    const rawAddress = await sapling.getRawPaymentAddressFromIncomingViewingKey(
      Buffer.from(vector.ivk, 'hex'),
      Buffer.from(vector.diversifier, 'hex')
    );

    assert.equal(rawAddress.toString('hex'), `${vector.diversifier}${vector.pkd}`);
    assert.equal(
      await sapling.verifyCommitment(
        Buffer.from(vector.cmu, 'hex'),
        rawAddress,
        vector.value,
        Buffer.from(vector.rcm, 'hex')
      ),
      true
    );
  }
});
