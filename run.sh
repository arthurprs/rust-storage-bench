export RUST_LOG=${RUST_LOG:-warn}
sleep=5
minutes=2

# for vsize in 512; do
#     for db in fjall sled canopy-db; do
#         cargo run --bin worker --release -- --out task_f_rand_${db}_${vsize}v.jsonl --workload task-f --backend ${db} --minutes ${minutes} --value-size ${vsize} --items 5000000 --random
#         rm -rf .data
#         sleep ${sleep}
#     done
# done

for vsize in 512; do 
    for db in fjall redb sled persy canopy-db; do
        cargo run --bin worker --release -- --out task_g_rand_${db}_${vsize}v.jsonl --workload task-g --backend ${db} --minutes ${minutes} --value-size ${vsize} --items 10000 --random
        rm -rf .data
        sleep ${sleep}
    done
done

for vsize in 512; do
    for db in fjall redb sled persy canopy-db; do
        cargo run --bin worker --release -- --out task_g_seq_${db}_${vsize}v.jsonl --workload task-g --backend ${db} --minutes ${minutes} --value-size ${vsize} --items 10000
        rm -rf .data
        sleep ${sleep}
    done
done

minutes=1

for vsize in 512; do
    for db in fjall redb persy canopy-db heed; do
        cargo run --bin worker --release -- --out task_g_sync_rand_${db}_${vsize}v.jsonl --workload task-g --backend ${db} --minutes ${minutes} --value-size ${vsize} --items 10000 --random --fsync
        rm -rf .data
        sleep ${sleep}
    done
done
