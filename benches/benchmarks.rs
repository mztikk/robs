use criterion::{Criterion, criterion_group, criterion_main};
use itertools::Itertools;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use robs::signature::Signature;
use std::hint::black_box;

fn find_signature(search_region: &[u8], signature: &Signature) -> Option<usize> {
    let first_index = signature.first_byte?;
    let first_item = signature.pattern[first_index];
    let mask_len = signature.mask.len();

    search_region
        .iter()
        .positions(|&item| item == first_item)
        .filter(|&index| index + mask_len <= search_region.len())
        .find(|&index| {
            check_mask(
                &search_region[(index - first_index)..(index + mask_len)],
                signature,
            )
        })
        .map(|index| index + signature.offset)
}

fn find_signature_par_iter(search_region: &[u8], signature: &Signature) -> Option<usize> {
    let first_index = signature.first_byte?;
    let first_item = signature.pattern[first_index];
    let mask_len = signature.mask.len();

    search_region
        .par_iter()
        .positions(|&item| item == first_item)
        .filter(|&index| index + mask_len <= search_region.len())
        .find_any(|&index| {
            check_mask(
                &search_region[(index - first_index)..(index + mask_len)],
                signature,
            )
        })
        .map(|index| index + signature.offset)
}

fn check_mask(search_region: &[u8], signature: &Signature) -> bool {
    signature
        .matching_indices
        .iter()
        .all(|&i| search_region[i] == signature.pattern[i])
}

fn find_signature_slice_mask(search_region: &[u8], signature: &Signature) -> Option<usize> {
    let first_index = signature.first_byte?;
    let first_item = signature.pattern[first_index];
    let mask_len = signature.mask.len();

    search_region
        .iter()
        .positions(|&item| item == first_item)
        .filter(|&index| index + mask_len <= search_region.len())
        .find(|&index| {
            check_mask_slice_comparison(
                &search_region[(index - first_index)..(index + mask_len)],
                signature,
            )
        })
        .map(|index| index + signature.offset)
}

fn find_signature_par_iter_slice_mask(
    search_region: &[u8],
    signature: &Signature,
) -> Option<usize> {
    let first_index = signature.first_byte?;
    let first_item = signature.pattern[first_index];
    let mask_len = signature.mask.len();

    search_region
        .par_iter()
        .positions(|&item| item == first_item)
        .filter(|&index| index + mask_len <= search_region.len())
        .find_any(|&index| {
            check_mask_slice_comparison(
                &search_region[(index - first_index)..(index + mask_len)],
                signature,
            )
        })
        .map(|index| index + signature.offset)
}

fn find_signature_slice_mask_par_iter(
    search_region: &[u8],
    signature: &Signature,
) -> Option<usize> {
    let first_index = signature.first_byte?;
    let first_item = signature.pattern[first_index];
    let mask_len = signature.mask.len();

    search_region
        .iter()
        .positions(|&item| item == first_item)
        .filter(|&index| index + mask_len <= search_region.len())
        .find(|&index| {
            check_mask_slice_comparison_par_iter(
                &search_region[(index - first_index)..(index + mask_len)],
                signature,
            )
        })
        .map(|index| index + signature.offset)
}

fn check_mask_slice_comparison(search_region: &[u8], signature: &Signature) -> bool {
    signature
        .matching_index_ranges
        .iter()
        .all(|range| &search_region[range.clone()] == &signature.pattern[range.clone()])
}

fn check_mask_slice_comparison_par_iter(search_region: &[u8], signature: &Signature) -> bool {
    signature
        .matching_index_ranges
        .par_iter()
        .all(|range| &search_region[range.clone()] == &signature.pattern[range.clone()])
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("find_signature");

    let search_region = include_bytes!("search_region.bin");
    let signature = Signature::new("64 ?? 61 30 ?? ?? 36 64 65", 0).unwrap();
    let expected_index = 0x236;

    assert_eq!(
        find_signature(search_region, &signature),
        Some(expected_index)
    );
    assert_eq!(
        find_signature_par_iter(search_region, &signature),
        Some(expected_index)
    );
    assert_eq!(
        find_signature_slice_mask(search_region, &signature),
        Some(expected_index)
    );
    assert_eq!(
        find_signature_par_iter_slice_mask(search_region, &signature),
        Some(expected_index)
    );
    assert_eq!(
        find_signature_slice_mask_par_iter(search_region, &signature),
        Some(expected_index)
    );

    group.bench_function("find_signature", |b| {
        b.iter(|| find_signature(black_box(search_region), black_box(&signature)).unwrap())
    });
    group.bench_function("find_signature_par_iter", |b| {
        b.iter(|| find_signature_par_iter(black_box(search_region), black_box(&signature)).unwrap())
    });
    group.bench_function("find_signature_slice_mask", |b| {
        b.iter(|| {
            find_signature_slice_mask(black_box(search_region), black_box(&signature)).unwrap()
        })
    });
    group.bench_function("find_signature_par_iter_slice_mask", |b| {
        b.iter(|| {
            find_signature_par_iter_slice_mask(black_box(search_region), black_box(&signature))
                .unwrap()
        })
    });
    group.bench_function("find_signature_slice_mask_par_iter", |b| {
        b.iter(|| {
            find_signature_slice_mask_par_iter(black_box(search_region), black_box(&signature))
                .unwrap()
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
